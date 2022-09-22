#[doc = "DCIC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DCIC Control Register"]
    pub DCICC: crate::RWRegister<u32>,
    #[doc = "DCIC Interrupt Control Register"]
    pub DCICIC: crate::RWRegister<u32>,
    #[doc = "DCIC Status Register"]
    pub DCICS: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC0: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS0: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS0: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS0: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC1: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS1: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS1: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS1: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC2: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS2: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS2: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS2: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC3: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS3: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS3: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS3: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC4: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS4: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS4: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS4: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC5: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS5: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS5: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS5: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC6: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS6: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS6: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS6: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC7: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS7: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS7: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS7: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC8: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS8: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS8: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS8: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC9: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS9: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS9: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS9: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC10: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS10: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS10: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS10: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC11: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS11: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS11: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS11: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC12: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS12: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS12: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS12: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC13: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS13: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS13: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS13: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC14: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS14: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS14: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS14: crate::RORegister<u32>,
    #[doc = "DCIC ROI Config Register"]
    pub DCICRC15: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Size Register"]
    pub DCICRS15: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Reference Signature Register"]
    pub DCICRRS15: crate::RWRegister<u32>,
    #[doc = "DCIC ROI Calculated Signature Register"]
    pub DCICRCS15: crate::RORegister<u32>,
}
#[doc = "DCIC Control Register"]
pub mod DCICC {
    #[doc = "Integrity Check enable. Main enable switch."]
    pub mod IC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const IC_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const IC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "DATA_EN_IN signal polarity."]
    pub mod DE_POL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active High."]
            pub const DE_POL_0: u32 = 0;
            #[doc = "Active Low."]
            pub const DE_POL_1: u32 = 0x01;
        }
    }
    #[doc = "HSYNC_IN signal polarity."]
    pub mod HSYNC_POL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active High."]
            pub const HSYNC_POL_0: u32 = 0;
            #[doc = "Active Low."]
            pub const HSYNC_POL_1: u32 = 0x01;
        }
    }
    #[doc = "VSYNC_IN signal polarity."]
    pub mod VSYNC_POL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active High."]
            pub const VSYNC_POL_0: u32 = 0;
            #[doc = "Active Low."]
            pub const VSYNC_POL_1: u32 = 0x01;
        }
    }
    #[doc = "DISP_CLK signal polarity."]
    pub mod CLK_POL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted (default)."]
            pub const CLK_POL_0: u32 = 0;
            #[doc = "Inverted."]
            pub const CLK_POL_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC Interrupt Control Register"]
pub mod DCICIC {
    #[doc = "Error Interrupt mask. Can be changed only while FREEZE_MASK = 0."]
    pub mod EI_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask disabled - Interrupt assertion enabled"]
            pub const EI_MASK_0: u32 = 0;
            #[doc = "Mask enabled - Interrupt assertion disabled"]
            pub const EI_MASK_1: u32 = 0x01;
        }
    }
    #[doc = "Functional Interrupt mask. Can be changed only while FREEZE_MASK = 0."]
    pub mod FI_MASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask disabled - Interrupt assertion enabled"]
            pub const FI_MASK_0: u32 = 0;
            #[doc = "Mask enabled - Interrupt assertion disabled"]
            pub const FI_MASK_1: u32 = 0x01;
        }
    }
    #[doc = "Disable change of interrupt masks. \"Sticky\" bit which can be set once and cleared by reset only."]
    pub mod FREEZE_MASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masks change allowed"]
            pub const FREEZE_MASK_0: u32 = 0;
            #[doc = "Masks are frozen"]
            pub const FREEZE_MASK_1: u32 = 0x01;
        }
    }
    #[doc = "External controller mismatch indication signal."]
    pub mod EXT_SIG_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const EXT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const EXT_SIG_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC Status Register"]
pub mod DCICS {
    #[doc = "Each set bit of this field indicates there was a mismatch at the appropriate ROIs signature during the last frame"]
    pub mod ROI_MATCH_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI calculated CRC matches expected signature"]
            pub const ROI_MATCH_STAT_0: u32 = 0;
            #[doc = "Mismatch at ROI calculated CRC"]
            pub const ROI_MATCH_STAT_1: u32 = 0x01;
        }
    }
    #[doc = "Error Interrupt status"]
    pub mod EI_STAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No pending Interrupt"]
            pub const EI_STAT_0: u32 = 0;
            #[doc = "Pending Interrupt"]
            pub const EI_STAT_1: u32 = 0x01;
        }
    }
    #[doc = "Functional Interrupt status. Write \"1\" to clear."]
    pub mod FI_STAT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No pending Interrupt"]
            pub const FI_STAT_0: u32 = 0;
            #[doc = "Pending Interrupt"]
            pub const FI_STAT_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC0 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS0 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS0 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS0 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC1 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS1 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS1 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS1 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC2 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS2 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS2 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS2 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC3 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS3 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS3 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS3 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC4 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS4 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS4 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS4 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC5 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS5 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS5 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS5 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC6 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS6 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS6 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS6 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC7 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS7 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS7 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS7 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC8 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS8 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS8 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS8 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC9 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS9 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS9 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS9 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC10 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS10 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS10 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS10 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC11 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS11 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS11 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS11 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC12 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS12 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS12 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS12 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC13 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS13 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS13 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS13 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC14 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS14 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS14 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS14 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Config Register"]
pub mod DCICRC15 {
    #[doc = "Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1"]
    pub mod START_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1"]
    pub mod START_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, the only parameter of the ROI that can be changed is the reference signature"]
    pub mod ROI_FREEZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROI configuration can be changed"]
            pub const ROI_FREEZE_0: u32 = 0;
            #[doc = "ROI configuration is frozen"]
            pub const ROI_FREEZE_1: u32 = 0x01;
        }
    }
    #[doc = "ROI tracking enable"]
    pub mod ROI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROI_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ROI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "DCIC ROI Size Register"]
pub mod DCICRS15 {
    #[doc = "Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1"]
    pub mod END_OFFSET_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1"]
    pub mod END_OFFSET_Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Reference Signature Register"]
pub mod DCICRRS15 {
    #[doc = "32-bit expected signature (CRC calculation result) for the ROI"]
    pub mod REFERENCE_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCIC ROI Calculated Signature Register"]
pub mod DCICRCS15 {
    #[doc = "32-bit actual signature (CRC calculation result) for the ROI during the last frame"]
    pub mod CALCULATED_SIGNATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
