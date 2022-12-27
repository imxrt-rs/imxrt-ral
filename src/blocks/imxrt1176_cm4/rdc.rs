#[doc = "RDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version Information"]
    pub VIR: crate::RORegister<u32>,
    _reserved0: [u8; 0x20],
    #[doc = "Status"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Interrupt and Control"]
    pub INTCTRL: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub INTSTAT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x01d0],
    #[doc = "Master Domain Assignment"]
    pub MDA: [crate::RWRegister<u32>; 12usize],
    _reserved2: [u8; 0x01d0],
    #[doc = "Peripheral Domain Access Permissions"]
    pub PDAP: [crate::RWRegister<u32>; 128usize],
    _reserved3: [u8; 0x0200],
    #[doc = "Memory Region Start Address"]
    pub MRSA0: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA0: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC0: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS0: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA1: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA1: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC1: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS1: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA2: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA2: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC2: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS2: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA3: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA3: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC3: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS3: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA4: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA4: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC4: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS4: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA5: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA5: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC5: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS5: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA6: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA6: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC6: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS6: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA7: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA7: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC7: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS7: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA8: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA8: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC8: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS8: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA9: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA9: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC9: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS9: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA10: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA10: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC10: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS10: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA11: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA11: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC11: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS11: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA12: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA12: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC12: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS12: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA13: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA13: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC13: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS13: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA14: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA14: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC14: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS14: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA15: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA15: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC15: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS15: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA16: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA16: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC16: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS16: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA17: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA17: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC17: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS17: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA18: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA18: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC18: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS18: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA19: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA19: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC19: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS19: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA20: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA20: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC20: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS20: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA21: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA21: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC21: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS21: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA22: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA22: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC22: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS22: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA23: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA23: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC23: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS23: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA24: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA24: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC24: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS24: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA25: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA25: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC25: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS25: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA26: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA26: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC26: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS26: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA27: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA27: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC27: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS27: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA28: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA28: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC28: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS28: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA29: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA29: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC29: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS29: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA30: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA30: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC30: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS30: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA31: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA31: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC31: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS31: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA32: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA32: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC32: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS32: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA33: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA33: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC33: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS33: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA34: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA34: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC34: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS34: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA35: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA35: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC35: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS35: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA36: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA36: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC36: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS36: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA37: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA37: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC37: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS37: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA38: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA38: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC38: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS38: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA39: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA39: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC39: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS39: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA40: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA40: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC40: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS40: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA41: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA41: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC41: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS41: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA42: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA42: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC42: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS42: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA43: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA43: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC43: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS43: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA44: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA44: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC44: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS44: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA45: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA45: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC45: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS45: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA46: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA46: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC46: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS46: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA47: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA47: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC47: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS47: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA48: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA48: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC48: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS48: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA49: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA49: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC49: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS49: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA50: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA50: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC50: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS50: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA51: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA51: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC51: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS51: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA52: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA52: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC52: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS52: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA53: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA53: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC53: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS53: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA54: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA54: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC54: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS54: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA55: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA55: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC55: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS55: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA56: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA56: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC56: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS56: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA57: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA57: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC57: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS57: crate::RWRegister<u32>,
    #[doc = "Memory Region Start Address"]
    pub MRSA58: crate::RWRegister<u32>,
    #[doc = "Memory Region End Address"]
    pub MREA58: crate::RWRegister<u32>,
    #[doc = "Memory Region Control"]
    pub MRC58: crate::RWRegister<u32>,
    #[doc = "Memory Region Violation Status"]
    pub MRVS58: crate::RWRegister<u32>,
}
#[doc = "Version Information"]
pub mod VIR {
    #[doc = "Number of Domains"]
    pub mod NDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Masters"]
    pub mod NMSTR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Peripherals"]
    pub mod NPER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Memory Regions"]
    pub mod NRGN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod STAT {
    #[doc = "Domain ID"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Domain Status"]
    pub mod PDS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power Down Domain is OFF"]
            pub const PDS_0: u32 = 0;
            #[doc = "Power Down Domain is ON"]
            pub const PDS_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt and Control"]
pub mod INTCTRL {
    #[doc = "Restoration Complete Interrupt"]
    pub mod RCI_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt Disabled"]
            pub const RCI_EN_0: u32 = 0;
            #[doc = "Interrupt Enabled"]
            pub const RCI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Status"]
pub mod INTSTAT {
    #[doc = "Interrupt Status"]
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Pending"]
            pub const INT_0: u32 = 0;
            #[doc = "Interrupt Pending"]
            pub const INT_1: u32 = 0x01;
        }
    }
}
#[doc = "Master Domain Assignment"]
pub mod MDA {
    #[doc = "Domain ID"]
    pub mod DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master assigned to Processing Domain 0"]
            pub const DID_0: u32 = 0;
            #[doc = "Master assigned to Processing Domain 1"]
            pub const DID_1: u32 = 0x01;
        }
    }
    #[doc = "Assignment Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Locked"]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked"]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Peripheral Domain Access Permissions"]
pub mod PDAP {
    #[doc = "Domain 0 Write Access"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Write Access"]
            pub const D0W_0: u32 = 0;
            #[doc = "Write Access Allowed"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Read Access"]
            pub const D0R_0: u32 = 0;
            #[doc = "Read Access Allowed"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Write Access"]
            pub const D1W_0: u32 = 0;
            #[doc = "Write Access Allowed"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Read Access"]
            pub const D1R_0: u32 = 0;
            #[doc = "Read Access Allowed"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Semaphore Required"]
    pub mod SREQ {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Semaphores have no effect"]
            pub const SREQ_0: u32 = 0;
            #[doc = "Semaphores are enforced"]
            pub const SREQ_1: u32 = 0x01;
        }
    }
    #[doc = "Peripheral Permissions Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Locked"]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked"]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA0 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA0 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC0 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS0 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA1 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA1 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC1 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS1 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA2 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA2 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC2 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS2 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA3 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA3 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC3 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS3 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA4 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA4 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC4 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS4 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA5 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA5 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC5 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS5 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA6 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA6 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC6 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS6 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA7 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA7 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC7 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS7 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA8 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA8 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC8 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS8 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA9 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA9 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC9 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS9 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA10 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA10 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC10 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS10 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA11 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA11 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC11 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS11 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA12 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA12 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC12 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS12 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA13 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA13 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC13 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS13 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA14 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA14 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC14 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS14 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA15 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA15 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC15 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS15 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA16 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA16 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC16 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS16 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA17 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA17 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC17 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS17 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA18 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA18 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC18 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS18 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA19 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA19 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC19 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS19 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA20 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA20 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC20 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS20 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA21 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA21 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC21 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS21 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA22 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA22 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC22 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS22 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA23 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA23 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC23 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS23 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA24 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA24 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC24 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS24 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA25 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA25 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC25 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS25 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA26 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA26 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC26 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS26 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA27 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA27 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC27 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS27 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA28 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA28 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC28 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS28 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA29 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA29 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC29 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS29 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA30 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA30 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC30 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS30 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA31 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA31 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC31 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS31 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA32 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA32 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC32 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS32 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA33 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA33 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC33 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS33 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA34 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA34 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC34 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS34 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA35 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA35 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC35 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS35 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA36 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA36 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC36 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS36 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA37 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA37 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC37 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS37 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA38 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA38 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC38 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS38 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA39 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA39 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC39 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS39 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA40 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA40 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC40 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS40 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA41 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA41 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC41 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS41 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA42 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA42 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC42 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS42 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA43 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA43 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC43 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS43 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA44 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA44 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC44 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS44 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA45 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA45 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC45 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS45 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA46 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA46 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC46 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS46 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA47 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA47 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC47 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS47 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA48 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA48 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC48 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS48 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA49 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA49 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC49 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS49 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA50 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA50 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC50 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS50 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA51 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA51 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC51 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS51 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA52 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA52 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC52 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS52 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA53 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA53 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC53 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS53 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA54 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA54 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC54 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS54 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA55 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA55 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC55 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS55 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA56 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA56 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC56 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS56 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA57 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA57 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC57 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS57 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Start Address"]
pub mod MRSA58 {
    #[doc = "Start address for memory region"]
    pub mod SADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region End Address"]
pub mod MREA58 {
    #[doc = "Upper bound for memory region"]
    pub mod EADR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Region Control"]
pub mod MRC58 {
    #[doc = "Domain 0 Write Access to Region"]
    pub mod D0W {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Write access to the memory region"]
            pub const D0W_0: u32 = 0;
            #[doc = "Processing Domain 0 has Write access to the memory region"]
            pub const D0W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 0 Read Access to Region"]
    pub mod D0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0 does not have Read access to the memory region"]
            pub const D0R_0: u32 = 0;
            #[doc = "Processing Domain 0 has Read access to the memory region"]
            pub const D0R_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Write Access to Region"]
    pub mod D1W {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Write access to the memory region"]
            pub const D1W_0: u32 = 0;
            #[doc = "Processing Domain 1 has Write access to the memory region"]
            pub const D1W_1: u32 = 0x01;
        }
    }
    #[doc = "Domain 1 Read Access to Region"]
    pub mod D1R {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 1 does not have Read access to the memory region"]
            pub const D1R_0: u32 = 0;
            #[doc = "Processing Domain 1 has Read access to the memory region"]
            pub const D1R_1: u32 = 0x01;
        }
    }
    #[doc = "Region Enable"]
    pub mod ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory region is not defined or restricted."]
            pub const ENA_0: u32 = 0;
            #[doc = "Memory boundaries, domain permissions and controls are in effect."]
            pub const ENA_1: u32 = 0x01;
        }
    }
    #[doc = "Region Lock"]
    pub mod LCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock. All fields in this register may be modified."]
            pub const LCK_0: u32 = 0;
            #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
            pub const LCK_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Region Violation Status"]
pub mod MRVS58 {
    #[doc = "Violating Domain ID"]
    pub mod VDID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing Domain 0"]
            pub const VDID_0: u32 = 0;
            #[doc = "Processing Domain 1"]
            pub const VDID_1: u32 = 0x01;
        }
    }
    #[doc = "Access Denied"]
    pub mod AD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Violating Address"]
    pub mod VADR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
