#[doc = "PUF"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PUF Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "PUF Key Index Register"]
    pub KEYINDEX: crate::RWRegister<u32>,
    #[doc = "PUF Key Size Register"]
    pub KEYSIZE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x14],
    #[doc = "PUF Status Register"]
    pub STAT: crate::RORegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "PUF Allow Register"]
    pub ALLOW: crate::RORegister<u32>,
    _reserved2: [u8; 0x14],
    #[doc = "PUF Key Input Register"]
    pub KEYINPUT: crate::WORegister<u32>,
    #[doc = "PUF Code Input Register"]
    pub CODEINPUT: crate::WORegister<u32>,
    #[doc = "PUF Code Output Register"]
    pub CODEOUTPUT: crate::RORegister<u32>,
    _reserved3: [u8; 0x14],
    #[doc = "PUF Key Output Index Register"]
    pub KEYOUTINDEX: crate::RORegister<u32>,
    #[doc = "PUF Key Output Register"]
    pub KEYOUTPUT: crate::RORegister<u32>,
    _reserved4: [u8; 0x74],
    #[doc = "PUF Interface Status Register"]
    pub IFSTAT: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "PUF Version Register"]
    pub VERSION: crate::RORegister<u32>,
    #[doc = "PUF Interrupt Enable"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "PUF Interrupt Status"]
    pub INTSTAT: crate::RWRegister<u32>,
    #[doc = "PUF Power Control Of RAM"]
    pub PWRCTRL: crate::RWRegister<u32>,
    #[doc = "PUF Configuration Register"]
    pub CFG: crate::RWRegister<u32>,
    _reserved6: [u8; 0xf0],
    #[doc = "PUF Key Manager Lock"]
    pub KEYLOCK: crate::RWRegister<u32>,
    #[doc = "PUF Key Manager Enable"]
    pub KEYENABLE: crate::RWRegister<u32>,
    #[doc = "PUF Key Manager Reset"]
    pub KEYRESET: crate::RWRegister<u32>,
    #[doc = "PUF Index Block Key Output"]
    pub IDXBLK: crate::RWRegister<u32>,
    #[doc = "PUF Index Block Key Output"]
    pub IDXBLK_DP: crate::RWRegister<u32>,
    #[doc = "PUF Key Block 0 Mask Enable"]
    pub KEYMASK0: crate::RWRegister<u32>,
    #[doc = "PUF Key Block 1 Mask Enable"]
    pub KEYMASK1: crate::RWRegister<u32>,
    _reserved7: [u8; 0x38],
    #[doc = "PUF Index Block Setting Status Register"]
    pub IDXBLK_STATUS: crate::RORegister<u32>,
    #[doc = "PUF Key Manager Shift Status"]
    pub IDXBLK_SHIFT: crate::RORegister<u32>,
}
#[doc = "PUF Control Register"]
pub mod CTRL {
    #[doc = "Begin Zeroize operation for PUF and go to Error state"]
    pub mod ZEROIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Zeroize operation in progress"]
            pub const UNSET: u32 = 0;
            #[doc = "Zeroize operation in progress"]
            pub const SET: u32 = 0x01;
        }
    }
    #[doc = "Begin Enroll operation"]
    pub mod ENROLL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Enroll operation in progress"]
            pub const UNSET: u32 = 0;
            #[doc = "Enroll operation in progress"]
            pub const SET: u32 = 0x01;
        }
    }
    #[doc = "Begin Start operation"]
    pub mod START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Start operation in progress"]
            pub const UNSET: u32 = 0;
            #[doc = "Start operation in progress"]
            pub const SET: u32 = 0x01;
        }
    }
    #[doc = "Begin Set Intrinsic Key operation"]
    pub mod GENERATEKEY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Set Intrinsic Key operation in progress"]
            pub const UNSET: u32 = 0;
            #[doc = "Set Intrinsic Key operation in progress"]
            pub const SET: u32 = 0x01;
        }
    }
    #[doc = "Begin Set User Key operation"]
    pub mod SETKEY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Set Key operation in progress"]
            pub const UNSET: u32 = 0;
            #[doc = "Set Key operation in progress"]
            pub const SET: u32 = 0x01;
        }
    }
    #[doc = "Begin Get Key operation"]
    pub mod GETKEY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Get Key operation in progress"]
            pub const UNSET: u32 = 0;
            #[doc = "Get Key operation in progress"]
            pub const SET: u32 = 0x01;
        }
    }
}
#[doc = "PUF Key Index Register"]
pub mod KEYINDEX {
    #[doc = "PUF Key Index"]
    pub mod KEYIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USE INDEX0"]
            pub const INDEX0: u32 = 0;
            #[doc = "USE INDEX1"]
            pub const INDEX1: u32 = 0x01;
            #[doc = "USE INDEX2"]
            pub const INDEX2: u32 = 0x02;
            #[doc = "USE INDEX3"]
            pub const INDEX3: u32 = 0x03;
            #[doc = "USE INDEX4"]
            pub const INDEX4: u32 = 0x04;
            #[doc = "USE INDEX5"]
            pub const INDEX5: u32 = 0x05;
            #[doc = "USE INDEX6"]
            pub const INDEX6: u32 = 0x06;
            #[doc = "USE INDEX7"]
            pub const INDEX7: u32 = 0x07;
            #[doc = "USE INDEX8"]
            pub const INDEX8: u32 = 0x08;
            #[doc = "USE INDEX9"]
            pub const INDEX9: u32 = 0x09;
            #[doc = "USE INDEX10"]
            pub const INDEX10: u32 = 0x0a;
            #[doc = "USE INDEX11"]
            pub const INDEX11: u32 = 0x0b;
            #[doc = "USE INDEX12"]
            pub const INDEX12: u32 = 0x0c;
            #[doc = "USE INDEX13"]
            pub const INDEX13: u32 = 0x0d;
            #[doc = "USE INDEX14"]
            pub const INDEX14: u32 = 0x0e;
            #[doc = "USE INDEX15"]
            pub const INDEX15: u32 = 0x0f;
        }
    }
}
#[doc = "PUF Key Size Register"]
pub mod KEYSIZE {
    #[doc = "PUF Key Size"]
    pub mod KEYSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key Size is 512 Bytes and KC Size is 532 Bytes"]
            pub const SIZE64: u32 = 0;
            #[doc = "Key Size is 8 Bytes and KC Size is 52 Bytes"]
            pub const SIZE1: u32 = 0x01;
            #[doc = "Key Size is 16 Bytes and KC Size is 52 Bytes"]
            pub const SIZE2: u32 = 0x02;
            #[doc = "Key Size is 24 Bytes and KC Size is 52 Bytes"]
            pub const SIZE3: u32 = 0x03;
            #[doc = "Key Size is 32 Bytes and KC Size is 52 Bytes"]
            pub const SIZE4: u32 = 0x04;
            #[doc = "Key Size is 40 Bytes and KC Size is 84 Bytes"]
            pub const SIZE5: u32 = 0x05;
            #[doc = "Key Size is 48 Bytes and KC Size is 84 Bytes"]
            pub const SIZE6: u32 = 0x06;
            #[doc = "Key Size is 56 Bytes and KC Size is 84 Bytes"]
            pub const SIZE7: u32 = 0x07;
            #[doc = "Key Size is 64 Bytes and KC Size is 84 Bytes"]
            pub const SIZE8: u32 = 0x08;
            #[doc = "Key Size is 72 Bytes and KC Size is 116 Bytes"]
            pub const SIZE9: u32 = 0x09;
            #[doc = "Key Size is 80 Bytes and KC Size is 116 Bytes"]
            pub const SIZE10: u32 = 0x0a;
            #[doc = "Key Size is 88 Bytes and KC Size is 116 Bytes"]
            pub const SIZE11: u32 = 0x0b;
            #[doc = "Key Size is 96 Bytes and KC Size is 116 Bytes"]
            pub const SIZE12: u32 = 0x0c;
            #[doc = "Key Size is 104 Bytes and KC Size is 148 Bytes"]
            pub const SIZE13: u32 = 0x0d;
            #[doc = "Key Size is 112 Bytes and KC Size is 148 Bytes"]
            pub const SIZE14: u32 = 0x0e;
            #[doc = "Key Size is 120 Bytes and KC Size is 148 Bytes"]
            pub const SIZE15: u32 = 0x0f;
            #[doc = "Key Size is 128 Bytes and KC Size is 148 Bytes"]
            pub const SIZE16: u32 = 0x10;
            #[doc = "Key Size is 136 Bytes and KC Size is 180 Bytes"]
            pub const SIZE17: u32 = 0x11;
            #[doc = "Key Size is 144 Bytes and KC Size is 180 Bytes"]
            pub const SIZE18: u32 = 0x12;
            #[doc = "Key Size is 152 Bytes and KC Size is 180 Bytes"]
            pub const SIZE19: u32 = 0x13;
            #[doc = "Key Size is 160 Bytes and KC Size is 180 Bytes"]
            pub const SIZE20: u32 = 0x14;
            #[doc = "Key Size is 168 Bytes and KC Size is 212 Bytes"]
            pub const SIZE21: u32 = 0x15;
            #[doc = "Key Size is 176 Bytes and KC Size is 212 Bytes"]
            pub const SIZE22: u32 = 0x16;
            #[doc = "Key Size is 184 Bytes and KC Size is 212 Bytes"]
            pub const SIZE23: u32 = 0x17;
            #[doc = "Key Size is 192 Bytes and KC Size is 212 Bytes"]
            pub const SIZE24: u32 = 0x18;
            #[doc = "Key Size is 200 Bytes and KC Size is 244 Bytes"]
            pub const SIZE25: u32 = 0x19;
            #[doc = "Key Size is 208 Bytes and KC Size is 244 Bytes"]
            pub const SIZE26: u32 = 0x1a;
            #[doc = "Key Size is 216 Bytes and KC Size is 244 Bytes"]
            pub const SIZE27: u32 = 0x1b;
            #[doc = "Key Size is 224 Bytes and KC Size is 244 Bytes"]
            pub const SIZE28: u32 = 0x1c;
            #[doc = "Key Size is 232 Bytes and KC Size is 276 Bytes"]
            pub const SIZE29: u32 = 0x1d;
            #[doc = "Key Size is 240 Bytes and KC Size is 276 Bytes"]
            pub const SIZE30: u32 = 0x1e;
            #[doc = "Key Size is 248 Bytes and KC Size is 276 Bytes"]
            pub const SIZE31: u32 = 0x1f;
            #[doc = "Key Size is 256 Bytes and KC Size is 276 Bytes"]
            pub const SIZE32: u32 = 0x20;
            #[doc = "Key Size is 264 Bytes and KC Size is 308 Bytes"]
            pub const SIZE33: u32 = 0x21;
            #[doc = "Key Size is 272 Bytes and KC Size is 308 Bytes"]
            pub const SIZE34: u32 = 0x22;
            #[doc = "Key Size is 280 Bytes and KC Size is 308 Bytes"]
            pub const SIZE35: u32 = 0x23;
            #[doc = "Key Size is 288 Bytes and KC Size is 308 Bytes"]
            pub const SIZE36: u32 = 0x24;
            #[doc = "Key Size is 296 Bytes and KC Size is 340 Bytes"]
            pub const SIZE37: u32 = 0x25;
            #[doc = "Key Size is 304 Bytes and KC Size is 340 Bytes"]
            pub const SIZE38: u32 = 0x26;
            #[doc = "Key Size is 312 Bytes and KC Size is 340 Bytes"]
            pub const SIZE39: u32 = 0x27;
            #[doc = "Key Size is 320 Bytes and KC Size is 340 Bytes"]
            pub const SIZE40: u32 = 0x28;
            #[doc = "Key Size is 328 Bytes and KC Size is 372 Bytes"]
            pub const SIZE41: u32 = 0x29;
            #[doc = "Key Size is 336 Bytes and KC Size is 372 Bytes"]
            pub const SIZE42: u32 = 0x2a;
            #[doc = "Key Size is 344 Bytes and KC Size is 372 Bytes"]
            pub const SIZE43: u32 = 0x2b;
            #[doc = "Key Size is 352 Bytes and KC Size is 372 Bytes"]
            pub const SIZE44: u32 = 0x2c;
            #[doc = "Key Size is 360 Bytes and KC Size is 404 Bytes"]
            pub const SIZE45: u32 = 0x2d;
            #[doc = "Key Size is 368 Bytes and KC Size is 404 Bytes"]
            pub const SIZE46: u32 = 0x2e;
            #[doc = "Key Size is 376 Bytes and KC Size is 404 Bytes"]
            pub const SIZE47: u32 = 0x2f;
            #[doc = "Key Size is 384 Bytes and KC Size is 404 Bytes"]
            pub const SIZE48: u32 = 0x30;
            #[doc = "Key Size is 392 Bytes and KC Size is 436 Bytes"]
            pub const SIZE49: u32 = 0x31;
            #[doc = "Key Size is 400 Bytes and KC Size is 436 Bytes"]
            pub const SIZE50: u32 = 0x32;
            #[doc = "Key Size is 408 Bytes and KC Size is 436 Bytes"]
            pub const SIZE51: u32 = 0x33;
            #[doc = "Key Size is 416 Bytes and KC Size is 436 Bytes"]
            pub const SIZE52: u32 = 0x34;
            #[doc = "Key Size is 424 Bytes and KC Size is 468 Bytes"]
            pub const SIZE53: u32 = 0x35;
            #[doc = "Key Size is 432 Bytes and KC Size is 468 Bytes"]
            pub const SIZE54: u32 = 0x36;
            #[doc = "Key Size is 440 Bytes and KC Size is 468 Bytes"]
            pub const SIZE55: u32 = 0x37;
            #[doc = "Key Size is 448 Bytes and KC Size is 468 Bytes"]
            pub const SIZE56: u32 = 0x38;
            #[doc = "Key Size is 456 Bytes and KC Size is 500 Bytes"]
            pub const SIZE57: u32 = 0x39;
            #[doc = "Key Size is 464 Bytes and KC Size is 500 Bytes"]
            pub const SIZE58: u32 = 0x3a;
            #[doc = "Key Size is 472 Bytes and KC Size is 500 Bytes"]
            pub const SIZE59: u32 = 0x3b;
            #[doc = "Key Size is 480 Bytes and KC Size is 500 Bytes"]
            pub const SIZE60: u32 = 0x3c;
            #[doc = "Key Size is 488 Bytes and KC Size is 532 Bytes"]
            pub const SIZE61: u32 = 0x3d;
            #[doc = "Key Size is 496 Bytes and KC Size is 532 Bytes"]
            pub const SIZE62: u32 = 0x3e;
            #[doc = "Key Size is 504 Bytes and KC Size is 532 Bytes"]
            pub const SIZE63: u32 = 0x3f;
        }
    }
}
#[doc = "PUF Status Register"]
pub mod STAT {
    #[doc = "puf_busy"]
    pub mod BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IDLE"]
            pub const IDLE: u32 = 0;
            #[doc = "BUSY"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "puf_ok"]
    pub mod SUCCESS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last operation was unsuccessful"]
            pub const NO: u32 = 0;
            #[doc = "Last operation was successful"]
            pub const SUCCESSFUL: u32 = 0x01;
        }
    }
    #[doc = "puf_error"]
    pub mod ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF is not in the Error state"]
            pub const NO_IN_ERROR: u32 = 0;
            #[doc = "PUF is in the Error state"]
            pub const IN_ERROR: u32 = 0x01;
        }
    }
    #[doc = "KI_ir"]
    pub mod KEYINREQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request for next part of key"]
            pub const NOREQUEST: u32 = 0;
            #[doc = "Request for next part of key in KEYINPUT register"]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "KO_or"]
    pub mod KEYOUTAVAIL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Next part of key is not available"]
            pub const NOAVAILABLE: u32 = 0;
            #[doc = "Next part of key is available in KEYOUTPUT register"]
            pub const AVAILABLE: u32 = 0x01;
        }
    }
    #[doc = "CI_ir"]
    pub mod CODEINREQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request for next part of Activation Code/Key Code"]
            pub const NOREQUEST: u32 = 0;
            #[doc = "request for next part of Activation Code/Key Code in CODEINPUT register"]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "CO_or"]
    pub mod CODEOUTAVAIL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Next part of Activation Code/Key Code is not available"]
            pub const NOAVAILABLE: u32 = 0;
            #[doc = "Next part of Activation Code/Key Code is available in CODEOUTPUT register"]
            pub const AVAILABLE: u32 = 0x01;
        }
    }
}
#[doc = "PUF Allow Register"]
pub mod ALLOW {
    #[doc = "Allow Enroll operation"]
    pub mod ALLOWENROLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Specified operation is not currently allowed"]
            pub const NOALLOW: u32 = 0;
            #[doc = "Specified operation is allowed"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "Allow Start operation"]
    pub mod ALLOWSTART {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Specified operation is not currently allowed"]
            pub const NOALLOW: u32 = 0;
            #[doc = "Specified operation is allowed"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "Allow Set Key operations"]
    pub mod ALLOWSETKEY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Specified operation is not currently allowed"]
            pub const NOALLOW: u32 = 0;
            #[doc = "Specified operation is allowed"]
            pub const ALLOW: u32 = 0x01;
        }
    }
    #[doc = "Allow Get Key operation"]
    pub mod ALLOWGETKEY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Specified operation is not currently allowed"]
            pub const NOALLOW: u32 = 0;
            #[doc = "Specified operation is allowed"]
            pub const ALLOW: u32 = 0x01;
        }
    }
}
#[doc = "PUF Key Input Register"]
pub mod KEYINPUT {
    #[doc = "Key input data"]
    pub mod KEYIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Code Input Register"]
pub mod CODEINPUT {
    #[doc = "AC/KC input data"]
    pub mod CODEIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Code Output Register"]
pub mod CODEOUTPUT {
    #[doc = "AC/KC output data"]
    pub mod CODEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Key Output Index Register"]
pub mod KEYOUTINDEX {
    #[doc = "Output Key index"]
    pub mod KEYOUTIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Key Output Register"]
pub mod KEYOUTPUT {
    #[doc = "Key output data from a Get Key operation"]
    pub mod KEYOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Interface Status Register"]
pub mod IFSTAT {
    #[doc = "APB error has occurred"]
    pub mod ERROR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOERROR"]
            pub const NOERROR: u32 = 0;
            #[doc = "ERROR"]
            pub const ERROR: u32 = 0x01;
        }
    }
}
#[doc = "PUF Version Register"]
pub mod VERSION {
    #[doc = "Version of PUF"]
    pub mod VERSION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Interrupt Enable"]
pub mod INTEN {
    #[doc = "PUF Ready Interrupt Enable"]
    pub mod READYEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF ready interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PUF ready interrupt enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF_OK Interrupt Enable"]
    pub mod SUCCESSEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF successful interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PUF successful interrupt enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Error Interrupt Enable"]
    pub mod ERROREN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF error interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PUF error interrupt enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Key Input Register Interrupt Enable"]
    pub mod KEYINREQEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key interrupt request disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Key interrupt request enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Key Output Register Interrupt Enable"]
    pub mod KEYOUTAVAILEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key available interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Key available interrupt enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Code Input Register Interrupt Enable"]
    pub mod CODEINREQEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AC/KC interrupt request disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "AC/KC interrupt request enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Code Output Register Interrupt Enable"]
    pub mod CODEOUTAVAILEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AC/KC available interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "AC/KC available interrupt enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "PUF Interrupt Status"]
pub mod INTSTAT {
    #[doc = "PUF_FINISH Interrupt Status"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Indicates that last operation not finished"]
            pub const NOT_FINISHED: u32 = 0;
            #[doc = "Indicates that last operation is finished"]
            pub const FINISHED: u32 = 0x01;
        }
    }
    #[doc = "PUF_OK Interrupt Status"]
    pub mod SUCCESS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Indicates that last operation was not successful"]
            pub const UNSUCCESSFUL: u32 = 0;
            #[doc = "Indicates that last operation was successful"]
            pub const SUCCESSFUL: u32 = 0x01;
        }
    }
    #[doc = "PUF_ERROR Interrupt Status"]
    pub mod ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF is not in the Error state and operations can be performed"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "PUF is in the Error state and no operations can be performed"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "PUF Key Input Register Interrupt Status"]
    pub mod KEYINREQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request for next part of key"]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Request for next part of key"]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "PUF Key Output Register Interrupt Status"]
    pub mod KEYOUTAVAIL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Next part of key is not available"]
            pub const NOT_AVAILABLE: u32 = 0;
            #[doc = "Next part of key is available"]
            pub const AVAILABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Code Input Register Interrupt Status"]
    pub mod CODEINREQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request for next part of AC/KC"]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Request for next part of AC/KC"]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "PUF Code Output Register Interrupt Status"]
    pub mod CODEOUTAVAIL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Next part of AC/KC is not available"]
            pub const NOT_AVAILABLE: u32 = 0;
            #[doc = "Next part of AC/KC is available"]
            pub const AVAILABLE: u32 = 0x01;
        }
    }
}
#[doc = "PUF Power Control Of RAM"]
pub mod PWRCTRL {
    #[doc = "PUF RAM on"]
    pub mod RAM_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF RAM is in sleep mode (PUF operation disabled)"]
            pub const SLEEP: u32 = 0;
            #[doc = "PUF RAM is awake (normal PUF operation enabled)"]
            pub const WAKE: u32 = 0x01;
        }
    }
    #[doc = "Clock disable"]
    pub mod CK_DIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PUF RAM is clocked (normal PUF operation enabled)"]
            pub const ENABLE: u32 = 0;
            #[doc = "PUF RAM clock is gated/disabled (PUF operation disabled)"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "RAM initialization"]
    pub mod RAM_INITN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset the PUF RAM (PUF operation disabled)"]
            pub const RESET: u32 = 0;
            #[doc = "Do not reset the PUF RAM (normal PUF operation enabled)"]
            pub const DO_NOT_RESET: u32 = 0x01;
        }
    }
    #[doc = "PUF RAM power switches"]
    pub mod RAM_PSW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Configuration Register"]
pub mod CFG {
    #[doc = "PUF Block Set Key Disable"]
    pub mod PUF_BLOCK_SET_KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the Set Key state"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the Set Key state"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "PUF Block Enroll Disable"]
    pub mod PUF_BLOCK_ENROLL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the Enrollment state"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the Enrollment state"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "PUF Key Manager Lock"]
pub mod KEYLOCK {
    #[doc = "Lock Block 0"]
    pub mod LOCK0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SNVS Key block locked"]
            pub const LOCKED_00: u32 = 0;
            #[doc = "SNVS Key block locked"]
            pub const LOCKED_01: u32 = 0x01;
            #[doc = "SNVS Key block unlocked"]
            pub const UNLOCKED: u32 = 0x02;
            #[doc = "SNVS Key block locked"]
            pub const LOCKED_11: u32 = 0x03;
        }
    }
    #[doc = "Lock Block 1"]
    pub mod LOCK1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OTFAD Key block locked"]
            pub const LOCKED_00: u32 = 0;
            #[doc = "OTFAD Key block locked"]
            pub const LOCKED_01: u32 = 0x01;
            #[doc = "OTFAD Key block unlocked"]
            pub const UNLOCKED: u32 = 0x02;
            #[doc = "OTFAD Key block locked"]
            pub const LOCKED_11: u32 = 0x03;
        }
    }
}
#[doc = "PUF Key Manager Enable"]
pub mod KEYENABLE {
    #[doc = "Enable Block 0"]
    pub mod ENABLE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key block 0 disabled"]
            pub const DISABLED_00: u32 = 0;
            #[doc = "Key block 0 disabled"]
            pub const DISABLED_01: u32 = 0x01;
            #[doc = "Key block 0 enabled"]
            pub const ENABLED: u32 = 0x02;
            #[doc = "Key block 0 disabled"]
            pub const DISABLED_11: u32 = 0x03;
        }
    }
    #[doc = "Enable Block 1"]
    pub mod ENABLE1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key block 1 disabled"]
            pub const DISABLED_00: u32 = 0;
            #[doc = "Key block 1 disabled"]
            pub const DISABLED_01: u32 = 0x01;
            #[doc = "Key block 1 enabled"]
            pub const ENABLED: u32 = 0x02;
            #[doc = "Key block 1 disabled"]
            pub const DISABLED_11: u32 = 0x03;
        }
    }
}
#[doc = "PUF Key Manager Reset"]
pub mod KEYRESET {
    #[doc = "Reset Block 0"]
    pub mod RESET0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not reset key block 0"]
            pub const NORESET_00: u32 = 0;
            #[doc = "Do not reset key block 0"]
            pub const NORESET_01: u32 = 0x01;
            #[doc = "Reset key block 0"]
            pub const RESET: u32 = 0x02;
            #[doc = "Do not reset key block 0"]
            pub const NORESET_11: u32 = 0x03;
        }
    }
    #[doc = "Reset Block 1"]
    pub mod RESET1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not reset key block 1"]
            pub const NORESET_00: u32 = 0;
            #[doc = "Do not reset key block 1"]
            pub const NORESET_01: u32 = 0x01;
            #[doc = "Reset key block 1"]
            pub const RESET: u32 = 0x02;
            #[doc = "Do not reset key block 1"]
            pub const NORESET_11: u32 = 0x03;
        }
    }
}
#[doc = "PUF Index Block Key Output"]
pub mod IDXBLK {
    #[doc = "idxblk0"]
    pub mod IDXBLK0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk1"]
    pub mod IDXBLK1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk2"]
    pub mod IDXBLK2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk3"]
    pub mod IDXBLK3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk4"]
    pub mod IDXBLK4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk5"]
    pub mod IDXBLK5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk6"]
    pub mod IDXBLK6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk7"]
    pub mod IDXBLK7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk8"]
    pub mod IDXBLK8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk9"]
    pub mod IDXBLK9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk10"]
    pub mod IDXBLK10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk11"]
    pub mod IDXBLK11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk12"]
    pub mod IDXBLK12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk13"]
    pub mod IDXBLK13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk14"]
    pub mod IDXBLK14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk15"]
    pub mod IDXBLK15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Index Block Key Output"]
pub mod IDXBLK_DP {
    #[doc = "idxblk_dp0"]
    pub mod IDXBLK_DP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp1"]
    pub mod IDXBLK_DP1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp2"]
    pub mod IDXBLK_DP2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp3"]
    pub mod IDXBLK_DP3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp4"]
    pub mod IDXBLK_DP4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp5"]
    pub mod IDXBLK_DP5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp6"]
    pub mod IDXBLK_DP6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp7"]
    pub mod IDXBLK_DP7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp8"]
    pub mod IDXBLK_DP8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp9"]
    pub mod IDXBLK_DP9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp10"]
    pub mod IDXBLK_DP10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp11"]
    pub mod IDXBLK_DP11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp12"]
    pub mod IDXBLK_DP12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp13"]
    pub mod IDXBLK_DP13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp14"]
    pub mod IDXBLK_DP14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_dp15"]
    pub mod IDXBLK_DP15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Key Block 0 Mask Enable"]
pub mod KEYMASK0 {
    #[doc = "KEYMASK0"]
    pub mod KEYMASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Key Block 1 Mask Enable"]
pub mod KEYMASK1 {
    #[doc = "KEYMASK1"]
    pub mod KEYMASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Index Block Setting Status Register"]
pub mod IDXBLK_STATUS {
    #[doc = "idxblk_status0"]
    pub mod IDXBLK_STATUS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status1"]
    pub mod IDXBLK_STATUS1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status2"]
    pub mod IDXBLK_STATUS2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status3"]
    pub mod IDXBLK_STATUS3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status4"]
    pub mod IDXBLK_STATUS4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status5"]
    pub mod IDXBLK_STATUS5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status6"]
    pub mod IDXBLK_STATUS6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status7"]
    pub mod IDXBLK_STATUS7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status8"]
    pub mod IDXBLK_STATUS8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status9"]
    pub mod IDXBLK_STATUS9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status10"]
    pub mod IDXBLK_STATUS10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status11"]
    pub mod IDXBLK_STATUS11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status12"]
    pub mod IDXBLK_STATUS12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status13"]
    pub mod IDXBLK_STATUS13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status14"]
    pub mod IDXBLK_STATUS14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "idxblk_status15"]
    pub mod IDXBLK_STATUS15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Key Manager Shift Status"]
pub mod IDXBLK_SHIFT {
    #[doc = "Index of key space in block 0"]
    pub mod IND_KEY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Index of key space in block 1"]
    pub mod IND_KEY1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
