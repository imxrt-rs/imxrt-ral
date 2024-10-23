#[doc = "EIM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Error Injection Module Configuration Register"]
    pub EIMCR: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Enable register"]
    pub EICHEN: crate::RWRegister<u32>,
    _reserved0: [u8; 0xf8],
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    pub EICHD0_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    pub EICHD0_WORD1: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 0, Word2"]
    pub EICHD0_WORD2: crate::RWRegister<u32>,
    _reserved1: [u8; 0x34],
    #[doc = "Error Injection Channel Descriptor 1, Word0"]
    pub EICHD1_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 1, Word1"]
    pub EICHD1_WORD1: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 1, Word2"]
    pub EICHD1_WORD2: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 1, Word3"]
    pub EICHD1_WORD3: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 1, Word4"]
    pub EICHD1_WORD4: crate::RWRegister<u32>,
    _reserved2: [u8; 0x2c],
    #[doc = "Error Injection Channel Descriptor 2, Word0"]
    pub EICHD2_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 2, Word1"]
    pub EICHD2_WORD1: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 2, Word2"]
    pub EICHD2_WORD2: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 2, Word3"]
    pub EICHD2_WORD3: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 2, Word4"]
    pub EICHD2_WORD4: crate::RWRegister<u32>,
    _reserved3: [u8; 0x2c],
    #[doc = "Error Injection Channel Descriptor 3, Word0"]
    pub EICHD3_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 3, Word1"]
    pub EICHD3_WORD1: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 3, Word2"]
    pub EICHD3_WORD2: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 3, Word3"]
    pub EICHD3_WORD3: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 3, Word4"]
    pub EICHD3_WORD4: crate::RWRegister<u32>,
    _reserved4: [u8; 0x2c],
    #[doc = "Error Injection Channel Descriptor 4, Word0"]
    pub EICHD4_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 4, Word1"]
    pub EICHD4_WORD1: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 4, Word2"]
    pub EICHD4_WORD2: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 4, Word3"]
    pub EICHD4_WORD3: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 4, Word4"]
    pub EICHD4_WORD4: crate::RWRegister<u32>,
}
#[doc = "Error Injection Module Configuration Register"]
pub mod EIMCR {
    #[doc = "Global Error Injection Enable"]
    pub mod GEIEN {
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
#[doc = "Error Injection Channel Enable register"]
pub mod EICHEN {
    #[doc = "Error Injection Channel 4 Enable"]
    pub mod EICH4EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 4"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 4"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Injection Channel 3 Enable"]
    pub mod EICH3EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 3"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 3"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Injection Channel 2 Enable"]
    pub mod EICH2EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 2"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 2"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Injection Channel 1 Enable"]
    pub mod EICH1EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 1"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 1"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Error Injection Channel 0 Enable"]
    pub mod EICH0EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 0"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 0"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word0"]
pub mod EICHD0_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word1"]
pub mod EICHD0_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word2"]
pub mod EICHD0_WORD2 {
    #[doc = "Data Mask Bytes 4-7"]
    pub mod B4_7DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word0"]
pub mod EICHD1_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word1"]
pub mod EICHD1_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word2"]
pub mod EICHD1_WORD2 {
    #[doc = "Data Mask Bytes 4-7"]
    pub mod B4_7DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word3"]
pub mod EICHD1_WORD3 {
    #[doc = "Data Mask Bytes 8-11"]
    pub mod B8_11DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word4"]
pub mod EICHD1_WORD4 {
    #[doc = "Data Mask Bytes 12-15"]
    pub mod B12_15DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word0"]
pub mod EICHD2_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word1"]
pub mod EICHD2_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word2"]
pub mod EICHD2_WORD2 {
    #[doc = "Data Mask Bytes 4-7"]
    pub mod B4_7DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word3"]
pub mod EICHD2_WORD3 {
    #[doc = "Data Mask Bytes 8-11"]
    pub mod B8_11DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word4"]
pub mod EICHD2_WORD4 {
    #[doc = "Data Mask Bytes 12-15"]
    pub mod B12_15DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word0"]
pub mod EICHD3_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word1"]
pub mod EICHD3_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word2"]
pub mod EICHD3_WORD2 {
    #[doc = "Data Mask Bytes 4-7"]
    pub mod B4_7DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word3"]
pub mod EICHD3_WORD3 {
    #[doc = "Data Mask Bytes 8-11"]
    pub mod B8_11DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word4"]
pub mod EICHD3_WORD4 {
    #[doc = "Data Mask Bytes 12-15"]
    pub mod B12_15DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word0"]
pub mod EICHD4_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word1"]
pub mod EICHD4_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word2"]
pub mod EICHD4_WORD2 {
    #[doc = "Data Mask Bytes 4-7"]
    pub mod B4_7DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word3"]
pub mod EICHD4_WORD3 {
    #[doc = "Data Mask Bytes 8-11"]
    pub mod B8_11DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word4"]
pub mod EICHD4_WORD4 {
    #[doc = "Data Mask Bytes 12-15"]
    pub mod B12_15DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
