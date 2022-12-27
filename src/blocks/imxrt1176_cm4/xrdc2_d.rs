#[doc = "XRDC2"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control Register"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub SR: crate::RORegister<u32>,
    _reserved0: [u8; 0x0ff8],
    #[doc = "no description available"]
    pub MSCI_MSAC_WK: [mscimsacwk::RegisterBlock; 128usize],
    _reserved1: [u8; 0x0c00],
    #[doc = "no description available"]
    pub MDACI: [mdaci::RegisterBlock; 32usize],
    #[doc = "no description available"]
    pub PACI: [paci::RegisterBlock; 8usize],
    #[doc = "no description available"]
    pub MRCI: [mrci::RegisterBlock; 32usize],
}
#[doc = "Module Control Register"]
pub mod MCR {
    #[doc = "Global Valid MDAC"]
    pub mod GVLDM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MDACs are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "MDACs are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Global Valid Access Control"]
    pub mod GVLDC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access controls are disabled, XRDC2 allows all transactions."]
            pub const DISABLED: u32 = 0;
            #[doc = "Access controls are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Global Configuration Lock"]
    pub mod GCL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled, registers can be written by any domain."]
            pub const DISABLED_00: u32 = 0;
            #[doc = "Lock disabled until the next reset, registers can be written by any domain."]
            pub const DISABLED_01: u32 = 0x01;
            #[doc = "Lock enabled, only the global configuration lock owner (SR\\[GCLO\\]) can write to registers."]
            pub const ENABLED_10: u32 = 0x02;
            #[doc = "Lock enabled, all registers are read only until the next reset."]
            pub const ENABLED_11: u32 = 0x03;
        }
    }
}
#[doc = "Status Register"]
pub mod SR {
    #[doc = "Domain Identifier Number"]
    pub mod DIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Revision Level"]
    pub mod HRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Global Configuration Lock Owner"]
    pub mod GCLO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod mscimsacwk {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Memory Slot Access Control"]
        pub MSC_MSAC_W0: crate::RWRegister<u32>,
        #[doc = "Memory Slot Access Control"]
        pub MSC_MSAC_W1: crate::RWRegister<u32>,
    }
    #[doc = "Memory Slot Access Control"]
    pub mod MSC_MSAC_W0 {
        #[doc = "Domain \"x\" access control policy"]
        pub mod D0ACP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D1ACP {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D2ACP {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D3ACP {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D4ACP {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D5ACP {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D6ACP {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D7ACP {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Exclusive Access Lock Owner"]
        pub mod EALO {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Memory Slot Access Control"]
    pub mod MSC_MSAC_W1 {
        #[doc = "Domain \"x\" access control policy"]
        pub mod D8ACP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D9ACP {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D10ACP {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D11ACP {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D12ACP {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D13ACP {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D14ACP {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Domain \"x\" access control policy"]
        pub mod D15ACP {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Exclusive Access Lock"]
        pub mod EAL {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Lock disabled."]
                pub const DISABLED_00: u32 = 0;
                #[doc = "Lock disabled until next reset."]
                pub const DISABLED_01: u32 = 0x01;
                #[doc = "Lock enabled, lock state = available."]
                pub const ENABLED_10: u32 = 0x02;
                #[doc = "Lock enabled, lock state = not available."]
                pub const ENABLED_11: u32 = 0x03;
            }
        }
        #[doc = "Descriptor Lock"]
        pub mod DL2 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Lock disabled, descriptor registers can be written."]
                pub const DISABLED_00: u32 = 0;
                #[doc = "Lock disabled until the next reset, descriptor registers can be written."]
                pub const DISABLED_01: u32 = 0x01;
                #[doc = "Lock enabled, only domain \"x\" can only update the DxACP field; no other fields can be written."]
                pub const ENABLED_10: u32 = 0x02;
                #[doc = "Lock enabled, descriptor registers are read-only until the next reset."]
                pub const ENABLED_11: u32 = 0x03;
            }
        }
        #[doc = "Valid"]
        pub mod VLD {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The MSAC assignment is invalid."]
                pub const INVALID: u32 = 0;
                #[doc = "The MSAC assignment is valid."]
                pub const VALID: u32 = 0x01;
            }
        }
    }
}
pub mod mdaci {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "no description available"]
        pub MDACI_MDAJ: [mdacimdaj::RegisterBlock; 32usize],
    }
    pub mod mdacimdaj {
        #[doc = "no description available"]
        #[repr(C)]
        pub struct RegisterBlock {
            #[doc = "Master Domain Assignment"]
            pub MDAC_MDA_W0: crate::RWRegister<u32>,
            #[doc = "Master Domain Assignment"]
            pub MDAC_MDA_W1: crate::RWRegister<u32>,
        }
        #[doc = "Master Domain Assignment"]
        pub mod MDAC_MDA_W0 {
            #[doc = "Mask"]
            pub mod MASK {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0xffff << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Match"]
            pub mod MATCH {
                pub const offset: u32 = 16;
                pub const mask: u32 = 0xffff << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Master Domain Assignment"]
        pub mod MDAC_MDA_W1 {
            #[doc = "Domain Identifier"]
            pub mod DID {
                pub const offset: u32 = 16;
                pub const mask: u32 = 0x0f << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Privileged attribute"]
            pub mod PA {
                pub const offset: u32 = 24;
                pub const mask: u32 = 0x03 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Use the bus master's privileged/user attribute directly."]
                    pub const PA_00: u32 = 0;
                    #[doc = "Use the bus master's privileged/user attribute directly."]
                    pub const PA_01: u32 = 0x01;
                    #[doc = "Force the bus attribute for this master to user."]
                    pub const PA_10: u32 = 0x02;
                    #[doc = "Force the bus attribute for this master to privileged."]
                    pub const PA_11: u32 = 0x03;
                }
            }
            #[doc = "Secure attribute"]
            pub mod SA {
                pub const offset: u32 = 26;
                pub const mask: u32 = 0x03 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Use the bus master's secure/nonsecure attribute directly."]
                    pub const SA_00: u32 = 0;
                    #[doc = "Use the bus master's secure/nonsecure attribute directly."]
                    pub const SA_01: u32 = 0x01;
                    #[doc = "Force the bus attribute for this master to secure."]
                    pub const SA_10: u32 = 0x02;
                    #[doc = "Force the bus attribute for this master to nonsecure."]
                    pub const SA_11: u32 = 0x03;
                }
            }
            #[doc = "Descriptor Lock"]
            pub mod DL {
                pub const offset: u32 = 30;
                pub const mask: u32 = 0x01 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Lock disabled, registers can be written."]
                    pub const DISABLED: u32 = 0;
                    #[doc = "Lock enabled, registers are read-only until the next reset."]
                    pub const ENABLED: u32 = 0x01;
                }
            }
            #[doc = "Valid"]
            pub mod VLD {
                pub const offset: u32 = 31;
                pub const mask: u32 = 0x01 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "The MDA is invalid."]
                    pub const INVALID: u32 = 0;
                    #[doc = "The MDA is valid."]
                    pub const VALID: u32 = 0x01;
                }
            }
        }
    }
}
pub mod paci {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "no description available"]
        pub PACI_PDACJ: [pacipdacj::RegisterBlock; 256usize],
    }
    pub mod pacipdacj {
        #[doc = "no description available"]
        #[repr(C)]
        pub struct RegisterBlock {
            #[doc = "Peripheral Domain Access Control"]
            pub PAC_PDAC_W0: crate::RWRegister<u32>,
            #[doc = "Peripheral Domain Access Control"]
            pub PAC_PDAC_W1: crate::RWRegister<u32>,
        }
        #[doc = "Peripheral Domain Access Control"]
        pub mod PAC_PDAC_W0 {
            #[doc = "Domain \"x\" access control policy"]
            pub mod D0ACP {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D1ACP {
                pub const offset: u32 = 3;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D2ACP {
                pub const offset: u32 = 6;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D3ACP {
                pub const offset: u32 = 9;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D4ACP {
                pub const offset: u32 = 12;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D5ACP {
                pub const offset: u32 = 15;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D6ACP {
                pub const offset: u32 = 18;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D7ACP {
                pub const offset: u32 = 21;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Exclusive Access Lock Owner"]
            pub mod EALO {
                pub const offset: u32 = 24;
                pub const mask: u32 = 0x0f << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Peripheral Domain Access Control"]
        pub mod PAC_PDAC_W1 {
            #[doc = "Domain \"x\" access control policy"]
            pub mod D8ACP {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D9ACP {
                pub const offset: u32 = 3;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D10ACP {
                pub const offset: u32 = 6;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D11ACP {
                pub const offset: u32 = 9;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D12ACP {
                pub const offset: u32 = 12;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D13ACP {
                pub const offset: u32 = 15;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D14ACP {
                pub const offset: u32 = 18;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D15ACP {
                pub const offset: u32 = 21;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Exclusive Access Lock"]
            pub mod EAL {
                pub const offset: u32 = 24;
                pub const mask: u32 = 0x03 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Lock disabled."]
                    pub const DISABLED_00: u32 = 0;
                    #[doc = "Lock disabled until next reset."]
                    pub const DISABLED_01: u32 = 0x01;
                    #[doc = "Lock enabled, lock state = available."]
                    pub const ENABLED_10: u32 = 0x02;
                    #[doc = "Lock enabled, lock state = not available."]
                    pub const ENABLED_11: u32 = 0x03;
                }
            }
            #[doc = "Descriptor Lock"]
            pub mod DL2 {
                pub const offset: u32 = 29;
                pub const mask: u32 = 0x03 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Lock disabled, descriptor registers can be written.."]
                    pub const DISABLED_00: u32 = 0;
                    #[doc = "Lock disabled until the next reset, descriptor registers can be written.."]
                    pub const DISABLED_01: u32 = 0x01;
                    #[doc = "Lock enabled, only domain \"x\" can only update the DxACP field; no other fields can be written.."]
                    pub const ENABLED_10: u32 = 0x02;
                    #[doc = "Lock enabled, descriptor registers are read-only until the next reset."]
                    pub const ENABLED_11: u32 = 0x03;
                }
            }
            #[doc = "Valid"]
            pub mod VLD {
                pub const offset: u32 = 31;
                pub const mask: u32 = 0x01 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "The PDAC assignment is invalid."]
                    pub const INVALID: u32 = 0;
                    #[doc = "The PDAC assignment is valid."]
                    pub const VALID: u32 = 0x01;
                }
            }
        }
    }
}
pub mod mrci {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "no description available"]
        pub MRCI_MRGDJ: [mrcimrgdj::RegisterBlock; 32usize],
    }
    pub mod mrcimrgdj {
        #[doc = "no description available"]
        #[repr(C)]
        pub struct RegisterBlock {
            #[doc = "Memory Region Descriptor"]
            pub MRC_MRGD_W0: crate::RWRegister<u32>,
            #[doc = "Memory Region Descriptor"]
            pub MRC_MRGD_W1: crate::RWRegister<u32>,
            #[doc = "Memory Region Descriptor"]
            pub MRC_MRGD_W2: crate::RWRegister<u32>,
            #[doc = "Memory Region Descriptor"]
            pub MRC_MRGD_W3: crate::RWRegister<u32>,
            _reserved0: [u8; 0x04],
            #[doc = "Memory Region Descriptor"]
            pub MRC_MRGD_W5: crate::RWRegister<u32>,
            #[doc = "Memory Region Descriptor"]
            pub MRC_MRGD_W6: crate::RWRegister<u32>,
            _reserved1: [u8; 0x04],
        }
        #[doc = "Memory Region Descriptor"]
        pub mod MRC_MRGD_W0 {
            #[doc = "Start Address"]
            pub mod SRTADDR {
                pub const offset: u32 = 12;
                pub const mask: u32 = 0x000f_ffff << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Memory Region Descriptor"]
        pub mod MRC_MRGD_W1 {
            #[doc = "Start Address"]
            pub mod SRTADDR {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0x0f << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Memory Region Descriptor"]
        pub mod MRC_MRGD_W2 {
            #[doc = "End Address"]
            pub mod ENDADDR {
                pub const offset: u32 = 12;
                pub const mask: u32 = 0x000f_ffff << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Memory Region Descriptor"]
        pub mod MRC_MRGD_W3 {
            #[doc = "End Address"]
            pub mod ENDADDR {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0x0f << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Memory Region Descriptor"]
        pub mod MRC_MRGD_W5 {
            #[doc = "Domain \"x\" access control policy"]
            pub mod D0ACP {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D1ACP {
                pub const offset: u32 = 3;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D2ACP {
                pub const offset: u32 = 6;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D3ACP {
                pub const offset: u32 = 9;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D4ACP {
                pub const offset: u32 = 12;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D5ACP {
                pub const offset: u32 = 15;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D6ACP {
                pub const offset: u32 = 18;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D7ACP {
                pub const offset: u32 = 21;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Exclusive Access Lock Owner"]
            pub mod EALO {
                pub const offset: u32 = 24;
                pub const mask: u32 = 0x0f << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
        }
        #[doc = "Memory Region Descriptor"]
        pub mod MRC_MRGD_W6 {
            #[doc = "Domain \"x\" access control policy"]
            pub mod D8ACP {
                pub const offset: u32 = 0;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D9ACP {
                pub const offset: u32 = 3;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D10ACP {
                pub const offset: u32 = 6;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D11ACP {
                pub const offset: u32 = 9;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D12ACP {
                pub const offset: u32 = 12;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D13ACP {
                pub const offset: u32 = 15;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D14ACP {
                pub const offset: u32 = 18;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Domain \"x\" access control policy"]
            pub mod D15ACP {
                pub const offset: u32 = 21;
                pub const mask: u32 = 0x07 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {}
            }
            #[doc = "Exclusive Access Lock"]
            pub mod EAL {
                pub const offset: u32 = 24;
                pub const mask: u32 = 0x03 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Lock disabled."]
                    pub const DISABLED_00: u32 = 0;
                    #[doc = "Lock disabled until next reset."]
                    pub const DISABLED_01: u32 = 0x01;
                    #[doc = "Lock enabled, lock state = available."]
                    pub const ENABLED_10: u32 = 0x02;
                    #[doc = "Lock enabled, lock state = not available."]
                    pub const ENABLED_11: u32 = 0x03;
                }
            }
            #[doc = "Descriptor Lock"]
            pub mod DL2 {
                pub const offset: u32 = 29;
                pub const mask: u32 = 0x03 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "Lock disabled, descriptor registers can be written."]
                    pub const DISABLED_00: u32 = 0;
                    #[doc = "Lock disabled until the next reset, descriptor registers can be written."]
                    pub const DISABLED_01: u32 = 0x01;
                    #[doc = "Lock enabled, only domain \"x\" can only update the DxACP field; no other fields can be written."]
                    pub const ENABLED_10: u32 = 0x02;
                    #[doc = "Lock enabled, descriptor registers are read-only until the next reset."]
                    pub const ENABLED_11: u32 = 0x03;
                }
            }
            #[doc = "Valid"]
            pub mod VLD {
                pub const offset: u32 = 31;
                pub const mask: u32 = 0x01 << offset;
                pub mod R {}
                pub mod W {}
                pub mod RW {
                    #[doc = "The MRGD is invalid."]
                    pub const INVALID: u32 = 0;
                    #[doc = "The MRGD is valid."]
                    pub const VALID: u32 = 0x01;
                }
            }
        }
    }
}
