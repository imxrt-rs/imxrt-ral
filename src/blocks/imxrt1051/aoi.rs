#[doc = "AND/OR/INVERT module"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub BFCRT010: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub BFCRT230: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub BFCRT011: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub BFCRT231: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub BFCRT012: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub BFCRT232: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub BFCRT013: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub BFCRT233: crate::RWRegister<u16>,
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub mod BFCRT010 {
    #[doc = "Product term 1, D input configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT1_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT1_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT1_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT1_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, C input configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT1_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT1_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT1_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT1_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, B input configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT1_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT1_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT1_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT1_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, A input configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT1_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT1_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT1_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT1_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, D input configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT0_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT0_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT0_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT0_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, C input configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT0_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT0_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT0_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT0_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, B input configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT0_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT0_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT0_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT0_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, A input configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT0_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT0_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT0_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT0_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub mod BFCRT230 {
    #[doc = "Product term 3, D input configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT3_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT3_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT3_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT3_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, C input configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT3_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT3_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT3_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT3_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, B input configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT3_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT3_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT3_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT3_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, A input configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT3_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT3_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT3_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT3_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, D input configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT2_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT2_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT2_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT2_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, C input configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT2_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT2_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT2_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT2_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, B input configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT2_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT2_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT2_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT2_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, A input configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT2_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT2_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT2_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT2_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub mod BFCRT011 {
    #[doc = "Product term 1, D input configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT1_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT1_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT1_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT1_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, C input configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT1_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT1_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT1_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT1_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, B input configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT1_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT1_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT1_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT1_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, A input configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT1_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT1_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT1_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT1_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, D input configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT0_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT0_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT0_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT0_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, C input configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT0_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT0_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT0_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT0_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, B input configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT0_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT0_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT0_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT0_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, A input configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT0_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT0_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT0_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT0_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub mod BFCRT231 {
    #[doc = "Product term 3, D input configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT3_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT3_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT3_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT3_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, C input configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT3_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT3_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT3_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT3_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, B input configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT3_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT3_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT3_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT3_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, A input configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT3_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT3_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT3_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT3_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, D input configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT2_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT2_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT2_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT2_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, C input configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT2_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT2_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT2_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT2_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, B input configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT2_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT2_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT2_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT2_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, A input configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT2_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT2_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT2_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT2_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub mod BFCRT012 {
    #[doc = "Product term 1, D input configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT1_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT1_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT1_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT1_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, C input configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT1_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT1_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT1_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT1_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, B input configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT1_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT1_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT1_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT1_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, A input configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT1_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT1_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT1_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT1_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, D input configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT0_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT0_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT0_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT0_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, C input configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT0_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT0_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT0_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT0_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, B input configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT0_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT0_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT0_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT0_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, A input configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT0_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT0_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT0_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT0_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub mod BFCRT232 {
    #[doc = "Product term 3, D input configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT3_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT3_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT3_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT3_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, C input configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT3_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT3_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT3_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT3_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, B input configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT3_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT3_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT3_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT3_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, A input configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT3_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT3_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT3_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT3_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, D input configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT2_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT2_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT2_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT2_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, C input configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT2_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT2_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT2_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT2_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, B input configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT2_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT2_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT2_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT2_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, A input configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT2_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT2_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT2_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT2_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub mod BFCRT013 {
    #[doc = "Product term 1, D input configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT1_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT1_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT1_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT1_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, C input configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT1_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT1_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT1_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT1_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, B input configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT1_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT1_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT1_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT1_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 1, A input configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT1_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT1_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT1_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT1_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, D input configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT0_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT0_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT0_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT0_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, C input configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT0_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT0_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT0_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT0_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, B input configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT0_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT0_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT0_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT0_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 0, A input configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT0_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT0_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT0_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT0_AC_3: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub mod BFCRT233 {
    #[doc = "Product term 3, D input configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT3_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT3_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT3_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT3_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, C input configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT3_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT3_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT3_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT3_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, B input configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT3_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT3_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT3_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT3_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 3, A input configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT3_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT3_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT3_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT3_AC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, D input configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the D input in this product term to a logical zero"]
            pub const PT2_DC_0: u16 = 0;
            #[doc = "Pass the D input in this product term"]
            pub const PT2_DC_1: u16 = 0x01;
            #[doc = "Complement the D input in this product term"]
            pub const PT2_DC_2: u16 = 0x02;
            #[doc = "Force the D input in this product term to a logical one"]
            pub const PT2_DC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, C input configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the C input in this product term to a logical zero"]
            pub const PT2_CC_0: u16 = 0;
            #[doc = "Pass the C input in this product term"]
            pub const PT2_CC_1: u16 = 0x01;
            #[doc = "Complement the C input in this product term"]
            pub const PT2_CC_2: u16 = 0x02;
            #[doc = "Force the C input in this product term to a logical one"]
            pub const PT2_CC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, B input configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the B input in this product term to a logical zero"]
            pub const PT2_BC_0: u16 = 0;
            #[doc = "Pass the B input in this product term"]
            pub const PT2_BC_1: u16 = 0x01;
            #[doc = "Complement the B input in this product term"]
            pub const PT2_BC_2: u16 = 0x02;
            #[doc = "Force the B input in this product term to a logical one"]
            pub const PT2_BC_3: u16 = 0x03;
        }
    }
    #[doc = "Product term 2, A input configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force the A input in this product term to a logical zero"]
            pub const PT2_AC_0: u16 = 0;
            #[doc = "Pass the A input in this product term"]
            pub const PT2_AC_1: u16 = 0x01;
            #[doc = "Complement the A input in this product term"]
            pub const PT2_AC_2: u16 = 0x02;
            #[doc = "Force the A input in this product term to a logical one"]
            pub const PT2_AC_3: u16 = 0x03;
        }
    }
}
