#[doc = "System Control Block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "Auxiliary Control Register,"]
    pub ACTLR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0cf4],
    #[doc = "CPUID Base Register"]
    pub CPUID: crate::RORegister<u32>,
    #[doc = "Interrupt Control and State Register"]
    pub ICSR: crate::RWRegister<u32>,
    #[doc = "Vector Table Offset Register"]
    pub VTOR: crate::RWRegister<u32>,
    #[doc = "Application Interrupt and Reset Control Register"]
    pub AIRCR: crate::RWRegister<u32>,
    #[doc = "System Control Register"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "Configuration and Control Register"]
    pub CCR: crate::RWRegister<u32>,
    #[doc = "System Handler Priority Register 1"]
    pub SHPR1: crate::RWRegister<u32>,
    #[doc = "System Handler Priority Register 2"]
    pub SHPR2: crate::RWRegister<u32>,
    #[doc = "System Handler Priority Register 3"]
    pub SHPR3: crate::RWRegister<u32>,
    #[doc = "System Handler Control and State Register"]
    pub SHCSR: crate::RWRegister<u32>,
    #[doc = "Configurable Fault Status Register"]
    pub CFSR: crate::RWRegister<u32>,
    #[doc = "HardFault Status register"]
    pub HFSR: crate::RWRegister<u32>,
    #[doc = "Debug Fault Status Register"]
    pub DFSR: crate::RWRegister<u32>,
    #[doc = "MemManage Fault Address Register"]
    pub MMFAR: crate::RWRegister<u32>,
    #[doc = "BusFault Address Register"]
    pub BFAR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Processor Feature Register 0"]
    pub ID_PFR0: crate::RORegister<u32>,
    #[doc = "Processor Feature Register 1"]
    pub ID_PFR1: crate::RORegister<u32>,
    #[doc = "Debug Feature Register"]
    pub ID_DFR0: crate::RORegister<u32>,
    #[doc = "Auxiliary Feature Register"]
    pub ID_AFR0: crate::RORegister<u32>,
    #[doc = "Memory Model Feature Register 0"]
    pub ID_MMFR0: crate::RORegister<u32>,
    #[doc = "Memory Model Feature Register 1"]
    pub ID_MMFR1: crate::RORegister<u32>,
    #[doc = "Memory Model Feature Register 2"]
    pub ID_MMFR2: crate::RORegister<u32>,
    #[doc = "Memory Model Feature Register 3"]
    pub ID_MMFR3: crate::RORegister<u32>,
    #[doc = "Instruction Set Attributes Register 0"]
    pub ID_ISAR0: crate::RORegister<u32>,
    #[doc = "Instruction Set Attributes Register 1"]
    pub ID_ISAR1: crate::RORegister<u32>,
    #[doc = "Instruction Set Attributes Register 2"]
    pub ID_ISAR2: crate::RORegister<u32>,
    #[doc = "Instruction Set Attributes Register 3"]
    pub ID_ISAR3: crate::RORegister<u32>,
    #[doc = "Instruction Set Attributes Register 4"]
    pub ID_ISAR4: crate::RORegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Cache Level ID register"]
    pub CLIDR: crate::RORegister<u32>,
    #[doc = "Cache Type register"]
    pub CTR: crate::RORegister<u32>,
    #[doc = "Cache Size ID Register"]
    pub CCSIDR: crate::RORegister<u32>,
    #[doc = "Cache Size Selection Register"]
    pub CSSELR: crate::RWRegister<u32>,
    #[doc = "Coprocessor Access Control Register"]
    pub CPACR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0174],
    #[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
    pub STIR: crate::WORegister<u32>,
    _reserved5: [u8; 0x4c],
    #[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
    pub ICIALLU: crate::WORegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "Instruction cache invalidate by address to PoU"]
    pub ICIMVAU: crate::WORegister<u32>,
    #[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
    pub DCIMVAC: crate::WORegister<u32>,
    #[doc = "Data cache invalidate by set/way"]
    pub DCISW: crate::WORegister<u32>,
    #[doc = "Data cache by address to PoU"]
    pub DCCMVAU: crate::WORegister<u32>,
    #[doc = "Data cache clean by address to PoC"]
    pub DCCMVAC: crate::WORegister<u32>,
    #[doc = "Data cache clean by set/way"]
    pub DCCSW: crate::WORegister<u32>,
    #[doc = "Data cache clean and invalidate by address to PoC"]
    pub DCCIMVAC: crate::WORegister<u32>,
    #[doc = "Data cache clean and invalidate by set/way"]
    pub DCCISW: crate::WORegister<u32>,
    _reserved7: [u8; 0x18],
    #[doc = "Instruction Tightly-Coupled Memory Control Register"]
    pub CM7_ITCMCR: crate::RWRegister<u32>,
    #[doc = "Data Tightly-Coupled Memory Control Register"]
    pub CM7_DTCMCR: crate::RWRegister<u32>,
    #[doc = "AHBP Control Register"]
    pub CM7_AHBPCR: crate::RWRegister<u32>,
    #[doc = "L1 Cache Control Register"]
    pub CM7_CACR: crate::RWRegister<u32>,
    #[doc = "AHB Slave Control Register"]
    pub CM7_AHBSCR: crate::RWRegister<u32>,
    _reserved8: [u8; 0x04],
    #[doc = "Auxiliary Bus Fault Status Register"]
    pub CM7_ABFSR: crate::RWRegister<u32>,
}
#[doc = "Auxiliary Control Register,"]
pub mod ACTLR {
    #[doc = "Disables folding of IT instructions."]
    pub mod DISFOLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISFOLD_0: u32 = 0;
        }
    }
    #[doc = "Disables FPU exception outputs."]
    pub mod FPEXCODIS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const FPEXCODIS_0: u32 = 0;
            #[doc = "FPU exception outputs are disabled."]
            pub const FPEXCODIS_1: u32 = 0x01;
        }
    }
    #[doc = "Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    pub mod DISRAMODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISRAMODE_0: u32 = 0;
            #[doc = "Dynamic disabled."]
            pub const DISRAMODE_1: u32 = 0x01;
        }
    }
    #[doc = "Disables ITM and DWT ATB flush."]
    pub mod DISITMATBFLUSH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITM and DWT ATB flush disabled, this bit is always 1."]
            pub const DISITMATBFLUSH_1: u32 = 0x01;
        }
    }
    #[doc = "Disables BTAC read."]
    pub mod DISBTACREAD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISBTACREAD_0: u32 = 0;
            #[doc = "BTAC is not used and only static branch prediction can occur."]
            pub const DISBTACREAD_1: u32 = 0x01;
        }
    }
    #[doc = "Disables BTAC allocate."]
    pub mod DISBTACALLOC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISBTACALLOC_0: u32 = 0;
            #[doc = "No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
            pub const DISBTACALLOC_1: u32 = 0x01;
        }
    }
    #[doc = "Disables critical AXI Read-Under-Read."]
    pub mod DISCRITAXIRUR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISCRITAXIRUR_0: u32 = 0;
            #[doc = "An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
            pub const DISCRITAXIRUR_1: u32 = 0x01;
        }
    }
    #[doc = "Disables dual-issued."]
    pub mod DISDI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISDI_0: u32 = 0;
            #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
            pub const DISDI_1: u32 = 0x01;
        }
    }
    #[doc = "Disables dual-issued."]
    pub mod DISISSCH1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISISSCH1_0: u32 = 0;
            #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
            pub const DISISSCH1_1: u32 = 0x01;
        }
    }
    #[doc = "Disables dynamic allocation of ADD and SUB instructions"]
    pub mod DISDYNADD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
            pub const DISDYNADD_0: u32 = 0;
            #[doc = "All ADD and SUB instructions are resolved in EX2."]
            pub const DISDYNADD_1: u32 = 0x01;
        }
    }
    #[doc = "Disables critical AXI read-under-write"]
    pub mod DISCRITAXIRUW {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation. This is backwards compatible with r0."]
            pub const DISCRITAXIRUW_0: u32 = 0;
            #[doc = "AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
            pub const DISCRITAXIRUW_1: u32 = 0x01;
        }
    }
    #[doc = "Disables critical AXI read-under-write"]
    pub mod DISFPUISSOPT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const DISFPUISSOPT_0: u32 = 0;
        }
    }
}
#[doc = "CPUID Base Register"]
pub mod CPUID {
    #[doc = "Indicates patch release: 0x0 = Patch 0"]
    pub mod REVISION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates part number"]
    pub mod PARTNO {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARCHITECTURE"]
    pub mod ARCHITECTURE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates processor revision: 0x2 = Revision 2"]
    pub mod VARIANT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Implementer code"]
    pub mod IMPLEMENTER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Control and State Register"]
pub mod ICSR {
    #[doc = "Active exception number"]
    pub mod VECTACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether there are preempted active exceptions"]
    pub mod RETTOBASE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "there are preempted active exceptions to execute"]
            pub const RETTOBASE_0: u32 = 0;
            #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception"]
            pub const RETTOBASE_1: u32 = 0x01;
        }
    }
    #[doc = "Exception number of the highest priority pending enabled exception"]
    pub mod VECTPENDING {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt pending flag, excluding NMI and Faults"]
    pub mod ISRPENDING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No external interrupt pending."]
            pub const ISRPENDING_0: u32 = 0;
            #[doc = "External interrupt pending."]
            pub const ISRPENDING_1: u32 = 0x01;
        }
    }
    #[doc = "SysTick exception clear-pending bit"]
    pub mod PENDSTCLR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no effect"]
            pub const PENDSTCLR_0: u32 = 0;
            #[doc = "removes the pending state from the SysTick exception"]
            pub const PENDSTCLR_1: u32 = 0x01;
        }
    }
    #[doc = "SysTick exception set-pending bit"]
    pub mod PENDSTSET {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "write: no effect; read: SysTick exception is not pending"]
            pub const PENDSTSET_0: u32 = 0;
            #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
            pub const PENDSTSET_1: u32 = 0x01;
        }
    }
    #[doc = "PendSV clear-pending bit"]
    pub mod PENDSVCLR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no effect"]
            pub const PENDSVCLR_0: u32 = 0;
            #[doc = "removes the pending state from the PendSV exception"]
            pub const PENDSVCLR_1: u32 = 0x01;
        }
    }
    #[doc = "PendSV set-pending bit"]
    pub mod PENDSVSET {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "write: no effect; read: PendSV exception is not pending"]
            pub const PENDSVSET_0: u32 = 0;
            #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
            pub const PENDSVSET_1: u32 = 0x01;
        }
    }
    #[doc = "NMI set-pending bit"]
    pub mod NMIPENDSET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "write: no effect; read: NMI exception is not pending"]
            pub const NMIPENDSET_0: u32 = 0;
            #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
            pub const NMIPENDSET_1: u32 = 0x01;
        }
    }
}
#[doc = "Vector Table Offset Register"]
pub mod VTOR {
    #[doc = "Vector table base offset"]
    pub mod TBLOFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod AIRCR {
    #[doc = "Writing 1 to this bit causes a local system reset"]
    pub mod VECTRESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const VECTRESET_0: u32 = 0;
            #[doc = "Causes a local system reset"]
            pub const VECTRESET_1: u32 = 0x01;
        }
    }
    #[doc = "Writing 1 to this bit clears all active state information for fixed and configurable exceptions."]
    pub mod VECTCLRACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const VECTCLRACTIVE_0: u32 = 0;
            #[doc = "Clears all active state information for fixed and configurable exceptions"]
            pub const VECTCLRACTIVE_1: u32 = 0x01;
        }
    }
    #[doc = "System reset request"]
    pub mod SYSRESETREQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no system reset request"]
            pub const SYSRESETREQ_0: u32 = 0;
            #[doc = "asserts a signal to the outer system that requests a reset"]
            pub const SYSRESETREQ_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    pub mod PRIGROUP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data endianness"]
    pub mod ENDIANNESS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Little-endian"]
            pub const ENDIANNESS_0: u32 = 0;
            #[doc = "Big-endian"]
            pub const ENDIANNESS_1: u32 = 0x01;
        }
    }
    #[doc = "Register key"]
    pub mod VECTKEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Control Register"]
pub mod SCR {
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    pub mod SLEEPONEXIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "o not sleep when returning to Thread mode"]
            pub const SLEEPONEXIT_0: u32 = 0;
            #[doc = "enter sleep, or deep sleep, on return from an ISR"]
            pub const SLEEPONEXIT_1: u32 = 0x01;
        }
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode"]
    pub mod SLEEPDEEP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "sleep"]
            pub const SLEEPDEEP_0: u32 = 0;
            #[doc = "deep sleep"]
            pub const SLEEPDEEP_1: u32 = 0x01;
        }
    }
    #[doc = "Send Event on Pending bit"]
    pub mod SEVONPEND {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
            pub const SEVONPEND_0: u32 = 0;
            #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
            pub const SEVONPEND_1: u32 = 0x01;
        }
    }
}
#[doc = "Configuration and Control Register"]
pub mod CCR {
    #[doc = "Indicates how the processor enters Thread mode"]
    pub mod NONBASETHRDENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "processor can enter Thread mode only when no exception is active"]
            pub const NONBASETHRDENA_0: u32 = 0;
            #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
            pub const NONBASETHRDENA_1: u32 = 0x01;
        }
    }
    #[doc = "Enables unprivileged software access to the STIR"]
    pub mod USERSETMPEND {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable"]
            pub const USERSETMPEND_0: u32 = 0;
            #[doc = "enable"]
            pub const USERSETMPEND_1: u32 = 0x01;
        }
    }
    #[doc = "Enables unaligned access traps"]
    pub mod UNALIGN_TRP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "do not trap unaligned halfword and word accesses"]
            pub const UNALIGN_TRP_0: u32 = 0;
            #[doc = "trap unaligned halfword and word accesses"]
            pub const UNALIGN_TRP_1: u32 = 0x01;
        }
    }
    #[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    pub mod DIV_0_TRP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "do not trap divide by 0"]
            pub const DIV_0_TRP_0: u32 = 0;
            #[doc = "trap divide by 0"]
            pub const DIV_0_TRP_1: u32 = 0x01;
        }
    }
    #[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    pub mod BFHFNMIGN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
            pub const BFHFNMIGN_0: u32 = 0;
            #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
            pub const BFHFNMIGN_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates stack alignment on exception entry"]
    pub mod STKALIGN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4-byte aligned"]
            pub const STKALIGN_0: u32 = 0;
            #[doc = "8-byte aligned"]
            pub const STKALIGN_1: u32 = 0x01;
        }
    }
    #[doc = "Enables L1 data cache."]
    pub mod DC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "L1 data cache disabled"]
            pub const DC_0: u32 = 0;
            #[doc = "L1 data cache enabled"]
            pub const DC_1: u32 = 0x01;
        }
    }
    #[doc = "Enables L1 instruction cache."]
    pub mod IC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "L1 instruction cache disabled"]
            pub const IC_0: u32 = 0;
            #[doc = "L1 instruction cache enabled"]
            pub const IC_1: u32 = 0x01;
        }
    }
    #[doc = "Always reads-as-one. It indicates branch prediction is enabled."]
    pub mod BP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Handler Priority Register 1"]
pub mod SHPR1 {
    #[doc = "Priority of system handler 4, MemManage"]
    pub mod PRI_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority of system handler 5, BusFault"]
    pub mod PRI_5 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority of system handler 6, UsageFault"]
    pub mod PRI_6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Handler Priority Register 2"]
pub mod SHPR2 {
    #[doc = "Priority of system handler 11, SVCall"]
    pub mod PRI_11 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Handler Priority Register 3"]
pub mod SHPR3 {
    #[doc = "Priority of system handler 14, PendSV"]
    pub mod PRI_14 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority of system handler 15, SysTick exception"]
    pub mod PRI_15 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Handler Control and State Register"]
pub mod SHCSR {
    #[doc = "MemManage exception active bit"]
    pub mod MEMFAULTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const MEMFAULTACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const MEMFAULTACT_1: u32 = 0x01;
        }
    }
    #[doc = "BusFault exception active bit"]
    pub mod BUSFAULTACT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const BUSFAULTACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const BUSFAULTACT_1: u32 = 0x01;
        }
    }
    #[doc = "UsageFault exception active bit"]
    pub mod USGFAULTACT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const USGFAULTACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const USGFAULTACT_1: u32 = 0x01;
        }
    }
    #[doc = "SVCall active bit"]
    pub mod SVCALLACT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const SVCALLACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const SVCALLACT_1: u32 = 0x01;
        }
    }
    #[doc = "Debug monitor active bit"]
    pub mod MONITORACT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const MONITORACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const MONITORACT_1: u32 = 0x01;
        }
    }
    #[doc = "PendSV exception active bit"]
    pub mod PENDSVACT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const PENDSVACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const PENDSVACT_1: u32 = 0x01;
        }
    }
    #[doc = "SysTick exception active bit"]
    pub mod SYSTICKACT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not active"]
            pub const SYSTICKACT_0: u32 = 0;
            #[doc = "exception is active"]
            pub const SYSTICKACT_1: u32 = 0x01;
        }
    }
    #[doc = "UsageFault exception pending bit"]
    pub mod USGFAULTPENDED {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not pending"]
            pub const USGFAULTPENDED_0: u32 = 0;
            #[doc = "exception is pending"]
            pub const USGFAULTPENDED_1: u32 = 0x01;
        }
    }
    #[doc = "MemManage exception pending bit"]
    pub mod MEMFAULTPENDED {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not pending"]
            pub const MEMFAULTPENDED_0: u32 = 0;
            #[doc = "exception is pending"]
            pub const MEMFAULTPENDED_1: u32 = 0x01;
        }
    }
    #[doc = "BusFault exception pending bit"]
    pub mod BUSFAULTPENDED {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not pending"]
            pub const BUSFAULTPENDED_0: u32 = 0;
            #[doc = "exception is pending"]
            pub const BUSFAULTPENDED_1: u32 = 0x01;
        }
    }
    #[doc = "SVCall pending bit"]
    pub mod SVCALLPENDED {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "exception is not pending"]
            pub const SVCALLPENDED_0: u32 = 0;
            #[doc = "exception is pending"]
            pub const SVCALLPENDED_1: u32 = 0x01;
        }
    }
    #[doc = "MemManage enable bit"]
    pub mod MEMFAULTENA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable the exception"]
            pub const MEMFAULTENA_0: u32 = 0;
            #[doc = "enable the exception"]
            pub const MEMFAULTENA_1: u32 = 0x01;
        }
    }
    #[doc = "BusFault enable bit"]
    pub mod BUSFAULTENA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable the exception"]
            pub const BUSFAULTENA_0: u32 = 0;
            #[doc = "enable the exception"]
            pub const BUSFAULTENA_1: u32 = 0x01;
        }
    }
    #[doc = "UsageFault enable bit"]
    pub mod USGFAULTENA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable the exception"]
            pub const USGFAULTENA_0: u32 = 0;
            #[doc = "enable the exception"]
            pub const USGFAULTENA_1: u32 = 0x01;
        }
    }
}
#[doc = "Configurable Fault Status Register"]
pub mod CFSR {
    #[doc = "Instruction access violation flag"]
    pub mod IACCVIOL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no instruction access violation fault"]
            pub const IACCVIOL_0: u32 = 0;
            #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
            pub const IACCVIOL_1: u32 = 0x01;
        }
    }
    #[doc = "Data access violation flag"]
    pub mod DACCVIOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no data access violation fault"]
            pub const DACCVIOL_0: u32 = 0;
            #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
            pub const DACCVIOL_1: u32 = 0x01;
        }
    }
    #[doc = "MemManage fault on unstacking for a return from exception"]
    pub mod MUNSTKERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no unstacking fault"]
            pub const MUNSTKERR_0: u32 = 0;
            #[doc = "unstack for an exception return has caused one or more access violations"]
            pub const MUNSTKERR_1: u32 = 0x01;
        }
    }
    #[doc = "MemManage fault on stacking for exception entry"]
    pub mod MSTKERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no stacking fault"]
            pub const MSTKERR_0: u32 = 0;
            #[doc = "stacking for an exception entry has caused one or more access violations"]
            pub const MSTKERR_1: u32 = 0x01;
        }
    }
    #[doc = "MemManage fault occurred during floating-point lazy state preservation"]
    pub mod MLSPERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
            pub const MLSPERR_0: u32 = 0;
            #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
            pub const MLSPERR_1: u32 = 0x01;
        }
    }
    #[doc = "MemManage Fault Address Register (MMFAR) valid flag"]
    pub mod MMARVALID {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "value in MMAR is not a valid fault address"]
            pub const MMARVALID_0: u32 = 0;
            #[doc = "MMAR holds a valid fault address"]
            pub const MMARVALID_1: u32 = 0x01;
        }
    }
    #[doc = "Instruction bus error"]
    pub mod IBUSERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no instruction bus error"]
            pub const IBUSERR_0: u32 = 0;
            #[doc = "instruction bus error"]
            pub const IBUSERR_1: u32 = 0x01;
        }
    }
    #[doc = "Precise data bus error"]
    pub mod PRECISERR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no precise data bus error"]
            pub const PRECISERR_0: u32 = 0;
            #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
            pub const PRECISERR_1: u32 = 0x01;
        }
    }
    #[doc = "Imprecise data bus error"]
    pub mod IMPRECISERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no imprecise data bus error"]
            pub const IMPRECISERR_0: u32 = 0;
            #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
            pub const IMPRECISERR_1: u32 = 0x01;
        }
    }
    #[doc = "BusFault on unstacking for a return from exception"]
    pub mod UNSTKERR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no unstacking fault"]
            pub const UNSTKERR_0: u32 = 0;
            #[doc = "unstack for an exception return has caused one or more BusFaults"]
            pub const UNSTKERR_1: u32 = 0x01;
        }
    }
    #[doc = "BusFault on stacking for exception entry"]
    pub mod STKERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no stacking fault"]
            pub const STKERR_0: u32 = 0;
            #[doc = "stacking for an exception entry has caused one or more BusFaults"]
            pub const STKERR_1: u32 = 0x01;
        }
    }
    #[doc = "Bus fault occurred during floating-point lazy state preservation"]
    pub mod LSPERR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No bus fault occurred during floating-point lazy state preservation"]
            pub const LSPERR_0: u32 = 0;
            #[doc = "A bus fault occurred during floating-point lazy state preservation"]
            pub const LSPERR_1: u32 = 0x01;
        }
    }
    #[doc = "BusFault Address Register (BFAR) valid flag"]
    pub mod BFARVALID {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "value in BFAR is not a valid fault address"]
            pub const BFARVALID_0: u32 = 0;
            #[doc = "BFAR holds a valid fault address"]
            pub const BFARVALID_1: u32 = 0x01;
        }
    }
    #[doc = "Undefined instruction UsageFault"]
    pub mod UNDEFINSTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no undefined instruction UsageFault"]
            pub const UNDEFINSTR_0: u32 = 0;
            #[doc = "the processor has attempted to execute an undefined instruction"]
            pub const UNDEFINSTR_1: u32 = 0x01;
        }
    }
    #[doc = "Invalid state UsageFault"]
    pub mod INVSTATE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no invalid state UsageFault"]
            pub const INVSTATE_0: u32 = 0;
            #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
            pub const INVSTATE_1: u32 = 0x01;
        }
    }
    #[doc = "Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    pub mod INVPC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no invalid PC load UsageFault"]
            pub const INVPC_0: u32 = 0;
            #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
            pub const INVPC_1: u32 = 0x01;
        }
    }
    #[doc = "No coprocessor UsageFault"]
    pub mod NOCP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no UsageFault caused by attempting to access a coprocessor"]
            pub const NOCP_0: u32 = 0;
            #[doc = "the processor has attempted to access a coprocessor"]
            pub const NOCP_1: u32 = 0x01;
        }
    }
    #[doc = "Unaligned access UsageFault"]
    pub mod UNALIGNED {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
            pub const UNALIGNED_0: u32 = 0;
            #[doc = "the processor has made an unaligned memory access"]
            pub const UNALIGNED_1: u32 = 0x01;
        }
    }
    #[doc = "Divide by zero UsageFault"]
    pub mod DIVBYZERO {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
            pub const DIVBYZERO_0: u32 = 0;
            #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
            pub const DIVBYZERO_1: u32 = 0x01;
        }
    }
}
#[doc = "HardFault Status register"]
pub mod HFSR {
    #[doc = "Indicates a BusFault on a vector table read during exception processing."]
    pub mod VECTTBL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no BusFault on vector table read"]
            pub const VECTTBL_0: u32 = 0;
            #[doc = "BusFault on vector table read"]
            pub const VECTTBL_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    pub mod FORCED {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no forced HardFault"]
            pub const FORCED_0: u32 = 0;
            #[doc = "forced HardFault"]
            pub const FORCED_1: u32 = 0x01;
        }
    }
    #[doc = "Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    pub mod DEBUGEVT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Debug event has occurred."]
            pub const DEBUGEVT_0: u32 = 0;
            #[doc = "Debug event has occurred. The Debug Fault Status Register has been updated."]
            pub const DEBUGEVT_1: u32 = 0x01;
        }
    }
}
#[doc = "Debug Fault Status Register"]
pub mod DFSR {
    #[doc = "Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
    pub mod HALTED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No active halt request debug event"]
            pub const HALTED_0: u32 = 0;
            #[doc = "Halt request debug event active"]
            pub const HALTED_1: u32 = 0x01;
        }
    }
    #[doc = "Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    pub mod BKPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No current breakpoint debug event"]
            pub const BKPT_0: u32 = 0;
            #[doc = "At least one current breakpoint debug event"]
            pub const BKPT_1: u32 = 0x01;
        }
    }
    #[doc = "Debug event generated by the DWT"]
    pub mod DWTTRAP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No current debug events generated by the DWT"]
            pub const DWTTRAP_0: u32 = 0;
            #[doc = "At least one current debug event generated by the DWT"]
            pub const DWTTRAP_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates triggering of a Vector catch"]
    pub mod VCATCH {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Vector catch triggered"]
            pub const VCATCH_0: u32 = 0;
            #[doc = "Vector catch triggered"]
            pub const VCATCH_1: u32 = 0x01;
        }
    }
    #[doc = "Debug event generated because of the assertion of an external debug request"]
    pub mod EXTERNAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No external debug request debug event"]
            pub const EXTERNAL_0: u32 = 0;
            #[doc = "External debug request debug event"]
            pub const EXTERNAL_1: u32 = 0x01;
        }
    }
}
#[doc = "MemManage Fault Address Register"]
pub mod MMFAR {
    #[doc = "Address of MemManage fault location"]
    pub mod ADDRESS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BusFault Address Register"]
pub mod BFAR {
    #[doc = "Address of the BusFault location"]
    pub mod ADDRESS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor Feature Register 0"]
pub mod ID_PFR0 {
    #[doc = "ARM instruction set support"]
    pub mod STATE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARMv7-M unused"]
            pub const STATE0_0: u32 = 0;
            #[doc = "ARMv7-M unused"]
            pub const STATE0_1: u32 = 0x01;
            #[doc = "ARMv7-M unused"]
            pub const STATE0_2: u32 = 0x02;
            #[doc = "Support for Thumb encoding including Thumb-2 technology, with all basic 16-bit and 32-bit instructions."]
            pub const STATE0_3: u32 = 0x03;
        }
    }
    #[doc = "Thumb instruction set support"]
    pub mod STATE1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The processor does not support the ARM instruction set."]
            pub const STATE1_0: u32 = 0;
            #[doc = "ARMv7-M unused"]
            pub const STATE1_1: u32 = 0x01;
        }
    }
    #[doc = "ARMv7-M unused"]
    pub mod STATE2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARMv7-M unused"]
    pub mod STATE3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor Feature Register 1"]
pub mod ID_PFR1 {
    #[doc = "M profile programmers' model"]
    pub mod PROGMODEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARMv7-M unused"]
            pub const PROGMODEL_0: u32 = 0;
            #[doc = "Two-stack programmers' model supported"]
            pub const PROGMODEL_2: u32 = 0x02;
        }
    }
}
#[doc = "Debug Feature Register"]
pub mod ID_DFR0 {
    #[doc = "Support for memory-mapped debug model for M profile processors"]
    pub mod DEBUGMODEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DEBUGMODEL_0: u32 = 0;
            #[doc = "Support for M profile Debug architecture, with memory-mapped access."]
            pub const DEBUGMODEL_1: u32 = 0x01;
        }
    }
}
#[doc = "Auxiliary Feature Register"]
pub mod ID_AFR0 {
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    pub mod IMPLEMENTATION_DEFINED0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    pub mod IMPLEMENTATION_DEFINED1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    pub mod IMPLEMENTATION_DEFINED2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    pub mod IMPLEMENTATION_DEFINED3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Model Feature Register 0"]
pub mod ID_MMFR0 {
    #[doc = "Indicates support for a PMSA"]
    pub mod PMSASUPPORT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const PMSASUPPORT_0: u32 = 0;
            #[doc = "ARMv7-M unused"]
            pub const PMSASUPPORT_1: u32 = 0x01;
            #[doc = "ARMv7-M unused"]
            pub const PMSASUPPORT_2: u32 = 0x02;
            #[doc = "PMSAv7, providing support for a base region and subregions."]
            pub const PMSASUPPORT_3: u32 = 0x03;
        }
    }
    #[doc = "Indicates the outermost shareability domain implemented"]
    pub mod OUTERMOST_SHAREABILITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Implemented as Non-cacheable"]
            pub const OUTERMOST_SHAREABILITY_0: u32 = 0;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_1: u32 = 0x01;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_2: u32 = 0x02;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_3: u32 = 0x03;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_4: u32 = 0x04;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_5: u32 = 0x05;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_6: u32 = 0x06;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_7: u32 = 0x07;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_8: u32 = 0x08;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_9: u32 = 0x09;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_10: u32 = 0x0a;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_11: u32 = 0x0b;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_12: u32 = 0x0c;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_13: u32 = 0x0d;
            #[doc = "ARMv7-M unused"]
            pub const OUTERMOST_SHAREABILITY_14: u32 = 0x0e;
            #[doc = "Shareability ignored."]
            pub const OUTERMOST_SHAREABILITY_15: u32 = 0x0f;
        }
    }
    #[doc = "Indicates the number of shareability levels implemented"]
    pub mod SHAREABILITY_LEVELS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One level of shareability implemented"]
            pub const SHAREABILITY_LEVELS_0: u32 = 0;
            #[doc = "ARMv7-M unused"]
            pub const SHAREABILITY_LEVELS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the support for Tightly Coupled Memory"]
    pub mod TCM_SUPPORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No tightly coupled memories implemented."]
            pub const TCM_SUPPORT_0: u32 = 0;
            #[doc = "Tightly coupled memories implemented with IMPLEMENTATION DEFINED control."]
            pub const TCM_SUPPORT_1: u32 = 0x01;
            #[doc = "ARMv7-M unused"]
            pub const TCM_SUPPORT_2: u32 = 0x02;
        }
    }
    #[doc = "Indicates the support for Auxiliary registers"]
    pub mod AUXILIARY_REGISTERS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const AUXILIARY_REGISTERS_0: u32 = 0;
            #[doc = "Support for Auxiliary Control Register only."]
            pub const AUXILIARY_REGISTERS_1: u32 = 0x01;
            #[doc = "ARMv7-M unused"]
            pub const AUXILIARY_REGISTERS_2: u32 = 0x02;
        }
    }
}
#[doc = "Memory Model Feature Register 1"]
pub mod ID_MMFR1 {
    #[doc = "Gives information about the implemented memory model and memory management support."]
    pub mod ID_MMFR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Model Feature Register 2"]
pub mod ID_MMFR2 {
    #[doc = "Indicates the support for Wait For Interrupt (WFI) stalling"]
    pub mod WFI_STALL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const WFI_STALL_0: u32 = 0;
            #[doc = "Support for WFI stalling"]
            pub const WFI_STALL_1: u32 = 0x01;
        }
    }
}
#[doc = "Memory Model Feature Register 3"]
pub mod ID_MMFR3 {
    #[doc = "Gives information about the implemented memory model and memory management support."]
    pub mod ID_MMFR3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Instruction Set Attributes Register 0"]
pub mod ID_ISAR0 {
    #[doc = "Indicates the supported Bit Counting instructions"]
    pub mod BITCOUNT_INSTRS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const BITCOUNT_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the CLZ instruction"]
            pub const BITCOUNT_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported BitField instructions"]
    pub mod BITFIELD_INSTRS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const BITFIELD_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the BFC, BFI, SBFX, and UBFX instructions"]
            pub const BITFIELD_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported combined Compare and Branch instructions"]
    pub mod CMPBRANCH_INSTRS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const CMPBRANCH_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the CBNZ and CBZ instructions"]
            pub const CMPBRANCH_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported Coprocessor instructions"]
    pub mod COPROC_INSTRS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, except for separately attributed architectures, for example the Floating-point extension"]
            pub const COPROC_INSTRS_0: u32 = 0;
            #[doc = "Adds support for generic CDP, LDC, MCR, MRC, and STC instructions"]
            pub const COPROC_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for generic CDP2, LDC2, MCR2, MRC2, and STC2 instructions"]
            pub const COPROC_INSTRS_2: u32 = 0x02;
            #[doc = "As for 2, and adds support for generic MCRR and MRRC instructions"]
            pub const COPROC_INSTRS_3: u32 = 0x03;
            #[doc = "As for 3, and adds support for generic MCRR2 and MRRC2 instructions"]
            pub const COPROC_INSTRS_4: u32 = 0x04;
        }
    }
    #[doc = "Indicates the supported Debug instructions"]
    pub mod DEBUG_INSTRS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const DEBUG_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the BKPT instruction"]
            pub const DEBUG_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported Divide instructions"]
    pub mod DIVIDE_INSTRS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const DIVIDE_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the SDIV and UDIV instructions"]
            pub const DIVIDE_INSTRS_1: u32 = 0x01;
        }
    }
}
#[doc = "Instruction Set Attributes Register 1"]
pub mod ID_ISAR1 {
    #[doc = "Indicates the supported Extend instructions"]
    pub mod EXTEND_INSTRS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const EXTEND_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the SXTB, SXTH, UXTB, and UXTH instructions"]
            pub const EXTEND_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the SXTAB, SXTAB16, SXTAH, SXTB16, UXTAB, UXTAB16, UXTAH, and UXTB16 instructions"]
            pub const EXTEND_INSTRS_2: u32 = 0x02;
        }
    }
    #[doc = "Indicates the supported IfThen instructions"]
    pub mod IFTHEN_INSTRS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const IFTHEN_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the IT instructions, and for the IT bits in the PSRs"]
            pub const IFTHEN_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the support for data-processing instructions with long immediate"]
    pub mod IMMEDIATE_INSTRS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const IMMEDIATE_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the ADDW, MOVW, MOVT, and SUBW instructions"]
            pub const IMMEDIATE_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported Interworking instructions"]
    pub mod INTERWORK_INSTRS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const INTERWORK_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the BX instruction, and the T bit in the PSR"]
            pub const INTERWORK_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the BLX instruction, and PC loads have BX-like behavior"]
            pub const INTERWORK_INSTRS_2: u32 = 0x02;
            #[doc = "ARMv7-M unused"]
            pub const INTERWORK_INSTRS_3: u32 = 0x03;
        }
    }
}
#[doc = "Instruction Set Attributes Register 2"]
pub mod ID_ISAR2 {
    #[doc = "Indicates the supported additional load and store instructions"]
    pub mod LOADSTORE_INSTRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const LOADSTORE_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the LDRD and STRD instructions"]
            pub const LOADSTORE_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported Memory Hint instructions"]
    pub mod MEMHINT_INSTRS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const MEMHINT_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the PLD instruction, ARMv7-M unused."]
            pub const MEMHINT_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, ARMv7-M unused."]
            pub const MEMHINT_INSTRS_2: u32 = 0x02;
            #[doc = "As for 1 or 2, and adds support for the PLI instruction."]
            pub const MEMHINT_INSTRS_3: u32 = 0x03;
        }
    }
    #[doc = "Indicates the support for multi-access interruptible instructions"]
    pub mod MULTIACCESSINT_INSTRS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported. This means the LDM and STM instructions are not interruptible. ARMv7-M unused."]
            pub const MULTIACCESSINT_INSTRS_0: u32 = 0;
            #[doc = "LDM and STM instructions are restartable."]
            pub const MULTIACCESSINT_INSTRS_1: u32 = 0x01;
            #[doc = "LDM and STM instructions are continuable."]
            pub const MULTIACCESSINT_INSTRS_2: u32 = 0x02;
        }
    }
    #[doc = "Indicates the supported additional Multiply instructions"]
    pub mod MULT_INSTRS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported. This means only MUL is supported. ARMv7-M unused."]
            pub const MULT_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the MLA instruction, ARMv7-M unused."]
            pub const MULT_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the MLS instruction."]
            pub const MULT_INSTRS_2: u32 = 0x02;
        }
    }
    #[doc = "Indicates the supported advanced signed Multiply instructions"]
    pub mod MULTS_INSTRS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const MULTS_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the SMULL and SMLAL instructions"]
            pub const MULTS_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the SMLABB, SMLABT, SMLALBB, SMLALBT, SMLALTB, SMLALTT, SMLATB, SMLATT, SMLAWB, SMLAWT, SMULBB, SMULBT, SMULTB, SMULTT, SMULWB, and SMULWT instructions."]
            pub const MULTS_INSTRS_2: u32 = 0x02;
            #[doc = "As for 2, and adds support for the SMLAD, SMLADX, SMLALD, SMLALDX, SMLSD, SMLSDX, SMLSLD, SMLSLDX, SMMLA, SMMLAR, SMMLS, SMMLSR, SMMUL, SMMULR, SMUAD, SMUADX, SMUSD, and SMUSDX instructions."]
            pub const MULTS_INSTRS_3: u32 = 0x03;
        }
    }
    #[doc = "Indicates the supported advanced unsigned Multiply instructions"]
    pub mod MULTU_INSTRS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const MULTU_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the UMULL and UMLAL instructions."]
            pub const MULTU_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the UMAAL instruction."]
            pub const MULTU_INSTRS_2: u32 = 0x02;
        }
    }
    #[doc = "Indicates the supported Reversal instructions"]
    pub mod REVERSAL_INSTRS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused"]
            pub const REVERSAL_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the REV, REV16, and REVSH instructions, ARMv7-M unused."]
            pub const REVERSAL_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the RBIT instruction."]
            pub const REVERSAL_INSTRS_2: u32 = 0x02;
        }
    }
}
#[doc = "Instruction Set Attributes Register 3"]
pub mod ID_ISAR3 {
    #[doc = "Indicates the supported Saturate instructions"]
    pub mod SATURATE_INSTRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported"]
            pub const SATURATE_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the QADD, QDADD, QDSUB, and QSUB instructions, and for the Q bit in the PSRs."]
            pub const SATURATE_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported SIMD instructions"]
    pub mod SIMD_INSTRS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const SIMD_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the SSAT and USAT instructions, and for the Q bit in the PSRs."]
            pub const SIMD_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the PKHBT, PKHTB, QADD16, QADD8, QASX, QSUB16, QSUB8, QSAX, SADD16, SADD8, SASX, SEL, SHADD16, SHADD8, SHASX, SHSUB16, SHSUB8, SHSAX, SSAT16, SSUB16, SSUB8, SSAX, SXTAB16, SXTB16, UADD16, UADD8, UASX, UHADD16, UHADD8, UHASX, UHSUB16, UHSUB8, UHSAX, UQADD16, UQADD8, UQASX, UQSUB16, UQSUB8, UQSAX, USAD8, USADA8, USAT16, USUB16, USUB8, USAX, UXTAB16, and UXTB16 instructions. Also adds support for the GE\\[3:0\\] bits in the PSRs."]
            pub const SIMD_INSTRS_3: u32 = 0x03;
        }
    }
    #[doc = "Indicates the supported SVC instructions"]
    pub mod SVC_INSTRS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const SVC_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the SVC instruction."]
            pub const SVC_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Together with the ID_ISAR4\\[SYNCHPRIM_INSTRS_FRAC\\] indicates the supported Synchronization Primitives"]
    pub mod SYNCHPRIM_INSTRS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the supported Table Branch instructions"]
    pub mod TABBRANCH_INSTRS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const TABBRANCH_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the TBB and TBH instructions."]
            pub const TABBRANCH_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported non flag-setting MOV instructions"]
    pub mod THUMBCOPY_INSTRS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const THUMBCOPY_INSTRS_0: u32 = 0;
            #[doc = "Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
            pub const THUMBCOPY_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported non flag-setting MOV instructions"]
    pub mod TRUENOP_INSTRS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const TRUENOP_INSTRS_0: u32 = 0;
            #[doc = "Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
            pub const TRUENOP_INSTRS_1: u32 = 0x01;
        }
    }
}
#[doc = "Instruction Set Attributes Register 4"]
pub mod ID_ISAR4 {
    #[doc = "Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix."]
    pub mod UNPRIV_INSTRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const UNPRIV_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the LDRBT, LDRT, STRBT, and STRT instructions."]
            pub const UNPRIV_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for the LDRHT, LDRSBT, LDRSHT, and STRHT instructions."]
            pub const UNPRIV_INSTRS_2: u32 = 0x02;
        }
    }
    #[doc = "Indicates the support for instructions with shifts"]
    pub mod WITHSHIFTS_INSTRS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nonzero shifts supported only in MOV and shift instructions."]
            pub const WITHSHIFTS_INSTRS_0: u32 = 0;
            #[doc = "Adds support for shifts of loads and stores over the range LSL 0-3."]
            pub const WITHSHIFTS_INSTRS_1: u32 = 0x01;
            #[doc = "As for 1, and adds support for other constant shift options, on loads, stores, and other instructions."]
            pub const WITHSHIFTS_INSTRS_3: u32 = 0x03;
            #[doc = "ARMv7-M unused."]
            pub const WITHSHIFTS_INSTRS_4: u32 = 0x04;
        }
    }
    #[doc = "Indicates the support for Writeback addressing modes"]
    pub mod WRITEBACK_INSTRS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Basic support. Only the LDM, STM, PUSH, and POP instructions support writeback addressing modes. ARMv7-M unused."]
            pub const WRITEBACK_INSTRS_0: u32 = 0;
            #[doc = "Adds support for all of the writeback addressing modes defined in the ARMv7-M architecture."]
            pub const WRITEBACK_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates the supported Barrier instructions"]
    pub mod BARRIER_INSTRS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const BARRIER_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the DMB, DSB, and ISB barrier instructions."]
            pub const BARRIER_INSTRS_1: u32 = 0x01;
        }
    }
    #[doc = "Together with the ID_ISAR3\\[SYNCHPRIM_INSTRS\\] indicates the supported Synchronization Primitives"]
    pub mod SYNCHPRIM_INSTRS_FRAC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the supported M profile instructions to modify the PSRs"]
    pub mod PSR_M_INSTRS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None supported, ARMv7-M unused."]
            pub const PSR_M_INSTRS_0: u32 = 0;
            #[doc = "Adds support for the M-profile forms of the CPS, MRS, and MSR instructions, to access the PSRs."]
            pub const PSR_M_INSTRS_1: u32 = 0x01;
        }
    }
}
#[doc = "Cache Level ID register"]
pub mod CLIDR {
    #[doc = "Indicate the type of cache implemented at level 1."]
    pub mod CL1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL1_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL1_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL1_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL1_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL1_4: u32 = 0x04;
        }
    }
    #[doc = "Indicate the type of cache implemented at level 2."]
    pub mod CL2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL2_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL2_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL2_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL2_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL2_4: u32 = 0x04;
        }
    }
    #[doc = "Indicate the type of cache implemented at level 3."]
    pub mod CL3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL3_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL3_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL3_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL3_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL3_4: u32 = 0x04;
        }
    }
    #[doc = "Indicate the type of cache implemented at level 4."]
    pub mod CL4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL4_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL4_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL4_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL4_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL4_4: u32 = 0x04;
        }
    }
    #[doc = "Indicate the type of cache implemented at level 5."]
    pub mod CL5 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL5_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL5_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL5_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL5_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL5_4: u32 = 0x04;
        }
    }
    #[doc = "Indicate the type of cache implemented at level 6."]
    pub mod CL6 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL6_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL6_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL6_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL6_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL6_4: u32 = 0x04;
        }
    }
    #[doc = "Indicate the type of cache implemented at level 7."]
    pub mod CL7 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No cache"]
            pub const CL7_0: u32 = 0;
            #[doc = "Instruction cache only"]
            pub const CL7_1: u32 = 0x01;
            #[doc = "Data cache only"]
            pub const CL7_2: u32 = 0x02;
            #[doc = "Separate instruction and data caches"]
            pub const CL7_3: u32 = 0x03;
            #[doc = "Unified cache"]
            pub const CL7_4: u32 = 0x04;
        }
    }
    #[doc = "Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ."]
    pub mod LOUIS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0"]
            pub const LOUIS_0: u32 = 0;
            #[doc = "1"]
            pub const LOUIS_1: u32 = 0x01;
            #[doc = "2"]
            pub const LOUIS_2: u32 = 0x02;
            #[doc = "3"]
            pub const LOUIS_3: u32 = 0x03;
            #[doc = "4"]
            pub const LOUIS_4: u32 = 0x04;
            #[doc = "5"]
            pub const LOUIS_5: u32 = 0x05;
            #[doc = "6"]
            pub const LOUIS_6: u32 = 0x06;
            #[doc = "7"]
            pub const LOUIS_7: u32 = 0x07;
        }
    }
    #[doc = "Level of Coherency for the cache hierarchy"]
    pub mod LOC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0"]
            pub const LOC_0: u32 = 0;
            #[doc = "1"]
            pub const LOC_1: u32 = 0x01;
            #[doc = "2"]
            pub const LOC_2: u32 = 0x02;
            #[doc = "3"]
            pub const LOC_3: u32 = 0x03;
            #[doc = "4"]
            pub const LOC_4: u32 = 0x04;
            #[doc = "5"]
            pub const LOC_5: u32 = 0x05;
            #[doc = "6"]
            pub const LOC_6: u32 = 0x06;
            #[doc = "7"]
            pub const LOC_7: u32 = 0x07;
        }
    }
    #[doc = "Level of Unification for the cache hierarchy"]
    pub mod LOU {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0"]
            pub const LOU_0: u32 = 0;
            #[doc = "1"]
            pub const LOU_1: u32 = 0x01;
            #[doc = "2"]
            pub const LOU_2: u32 = 0x02;
            #[doc = "3"]
            pub const LOU_3: u32 = 0x03;
            #[doc = "4"]
            pub const LOU_4: u32 = 0x04;
            #[doc = "5"]
            pub const LOU_5: u32 = 0x05;
            #[doc = "6"]
            pub const LOU_6: u32 = 0x06;
            #[doc = "7"]
            pub const LOU_7: u32 = 0x07;
        }
    }
}
#[doc = "Cache Type register"]
pub mod CTR {
    #[doc = "Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
    pub mod IMINLINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
    pub mod DMINLINE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
    pub mod ERG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
    pub mod CWG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the implemented CTR format."]
    pub mod FORMAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARMv7 format."]
            pub const FORMAT_4: u32 = 0x04;
        }
    }
}
#[doc = "Cache Size ID Register"]
pub mod CCSIDR {
    #[doc = "(Log2(Number of words in cache line)) - 2."]
    pub mod LINESIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The line length of 4 words."]
            pub const LINESIZE_0: u32 = 0;
            #[doc = "The line length of 8 words."]
            pub const LINESIZE_1: u32 = 0x01;
            #[doc = "The line length of 16 words."]
            pub const LINESIZE_2: u32 = 0x02;
            #[doc = "The line length of 32 words."]
            pub const LINESIZE_3: u32 = 0x03;
            #[doc = "The line length of 64 words."]
            pub const LINESIZE_4: u32 = 0x04;
            #[doc = "The line length of 128 words."]
            pub const LINESIZE_5: u32 = 0x05;
            #[doc = "The line length of 256 words."]
            pub const LINESIZE_6: u32 = 0x06;
            #[doc = "The line length of 512 words."]
            pub const LINESIZE_7: u32 = 0x07;
        }
    }
    #[doc = "(Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1. The associativity does not have to be a power of 2."]
    pub mod ASSOCIATIVITY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "(Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The number of sets does not have to be a power of 2."]
    pub mod NUMSETS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the cache level supports write-allocation"]
    pub mod WA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feature not supported"]
            pub const WA_0: u32 = 0;
            #[doc = "Feature supported"]
            pub const WA_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the cache level supports read-allocation"]
    pub mod RA {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feature not supported"]
            pub const RA_0: u32 = 0;
            #[doc = "Feature supported"]
            pub const RA_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the cache level supports write-back"]
    pub mod WB {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feature not supported"]
            pub const WB_0: u32 = 0;
            #[doc = "Feature supported"]
            pub const WB_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the cache level supports write-through"]
    pub mod WT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feature not supported"]
            pub const WT_0: u32 = 0;
            #[doc = "Feature supported"]
            pub const WT_1: u32 = 0x01;
        }
    }
}
#[doc = "Cache Size Selection Register"]
pub mod CSSELR {
    #[doc = "Instruction not data bit"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data or unified cache."]
            pub const IND_0: u32 = 0;
            #[doc = "Instruction cache."]
            pub const IND_1: u32 = 0x01;
        }
    }
    #[doc = "Cache level of required cache"]
    pub mod LEVEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level 1 cache."]
            pub const LEVEL_0: u32 = 0;
            #[doc = "Level 2 cache."]
            pub const LEVEL_1: u32 = 0x01;
            #[doc = "Level 3 cache."]
            pub const LEVEL_2: u32 = 0x02;
            #[doc = "Level 4 cache."]
            pub const LEVEL_3: u32 = 0x03;
            #[doc = "Level 5 cache."]
            pub const LEVEL_4: u32 = 0x04;
            #[doc = "Level 6 cache."]
            pub const LEVEL_5: u32 = 0x05;
            #[doc = "Level 7 cache."]
            pub const LEVEL_6: u32 = 0x06;
        }
    }
}
#[doc = "Coprocessor Access Control Register"]
pub mod CPACR {
    #[doc = "Access privileges for coprocessor 0."]
    pub mod CP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP0_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP0_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP0_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 1."]
    pub mod CP1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP1_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP1_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP1_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 2."]
    pub mod CP2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP2_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP2_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP2_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 3."]
    pub mod CP3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP3_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP3_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP3_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 4."]
    pub mod CP4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP4_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP4_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP4_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 5."]
    pub mod CP5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP5_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP5_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP5_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 6."]
    pub mod CP6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP6_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP6_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP6_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 7."]
    pub mod CP7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP7_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP7_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP7_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 10."]
    pub mod CP10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP10_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP10_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP10_3: u32 = 0x03;
        }
    }
    #[doc = "Access privileges for coprocessor 11."]
    pub mod CP11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
            pub const CP11_0: u32 = 0;
            #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
            pub const CP11_1: u32 = 0x01;
            #[doc = "Full access."]
            pub const CP11_3: u32 = 0x03;
        }
    }
}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod STIR {
    #[doc = "Indicates the interrupt to be triggered"]
    pub mod INTID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod ICIALLU {
    #[doc = "I-cache invalidate all to PoU"]
    pub mod ICIALLU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Instruction cache invalidate by address to PoU"]
pub mod ICIMVAU {
    #[doc = "I-cache invalidate by MVA to PoU"]
    pub mod ICIMVAU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
pub mod DCIMVAC {
    #[doc = "D-cache invalidate by MVA to PoC"]
    pub mod DCIMVAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache invalidate by set/way"]
pub mod DCISW {
    #[doc = "D-cache invalidate by set-way"]
    pub mod DCISW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache by address to PoU"]
pub mod DCCMVAU {
    #[doc = "D-cache clean by MVA to PoU"]
    pub mod DCCMVAU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache clean by address to PoC"]
pub mod DCCMVAC {
    #[doc = "D-cache clean by MVA to PoC"]
    pub mod DCCMVAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache clean by set/way"]
pub mod DCCSW {
    #[doc = "D-cache clean by set-way"]
    pub mod DCCSW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache clean and invalidate by address to PoC"]
pub mod DCCIMVAC {
    #[doc = "D-cache clean and invalidate by MVA to PoC"]
    pub mod DCCIMVAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data cache clean and invalidate by set/way"]
pub mod DCCISW {
    #[doc = "D-cache clean and invalidate by set-way"]
    pub mod DCCISW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Instruction Tightly-Coupled Memory Control Register"]
pub mod CM7_ITCMCR {
    #[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TCM disabled."]
            pub const EN_0: u32 = 0;
            #[doc = "TCM enabled."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    pub mod RMW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RMW disabled."]
            pub const RMW_0: u32 = 0;
            #[doc = "RMW enabled."]
            pub const RMW_1: u32 = 0x01;
        }
    }
    #[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    pub mod RETEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retry phase disabled."]
            pub const RETEN_0: u32 = 0;
            #[doc = "Retry phase enabled."]
            pub const RETEN_1: u32 = 0x01;
        }
    }
    #[doc = "TCM size. Indicates the size of the relevant TCM."]
    pub mod SZ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TCM implemented."]
            pub const SZ_0: u32 = 0;
            #[doc = "4KB."]
            pub const SZ_3: u32 = 0x03;
            #[doc = "8KB."]
            pub const SZ_4: u32 = 0x04;
            #[doc = "16KB."]
            pub const SZ_5: u32 = 0x05;
            #[doc = "32KB."]
            pub const SZ_6: u32 = 0x06;
            #[doc = "64KB."]
            pub const SZ_7: u32 = 0x07;
            #[doc = "128KB."]
            pub const SZ_8: u32 = 0x08;
            #[doc = "256KB."]
            pub const SZ_9: u32 = 0x09;
            #[doc = "512KB."]
            pub const SZ_10: u32 = 0x0a;
            #[doc = "1MB."]
            pub const SZ_11: u32 = 0x0b;
            #[doc = "2MB."]
            pub const SZ_12: u32 = 0x0c;
            #[doc = "4MB."]
            pub const SZ_13: u32 = 0x0d;
            #[doc = "8MB."]
            pub const SZ_14: u32 = 0x0e;
            #[doc = "16MB."]
            pub const SZ_15: u32 = 0x0f;
        }
    }
}
#[doc = "Data Tightly-Coupled Memory Control Register"]
pub mod CM7_DTCMCR {
    #[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TCM disabled."]
            pub const EN_0: u32 = 0;
            #[doc = "TCM enabled."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    pub mod RMW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RMW disabled."]
            pub const RMW_0: u32 = 0;
            #[doc = "RMW enabled."]
            pub const RMW_1: u32 = 0x01;
        }
    }
    #[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    pub mod RETEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retry phase disabled."]
            pub const RETEN_0: u32 = 0;
            #[doc = "Retry phase enabled."]
            pub const RETEN_1: u32 = 0x01;
        }
    }
    #[doc = "TCM size. Indicates the size of the relevant TCM."]
    pub mod SZ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TCM implemented."]
            pub const SZ_0: u32 = 0;
            #[doc = "4KB."]
            pub const SZ_3: u32 = 0x03;
            #[doc = "8KB."]
            pub const SZ_4: u32 = 0x04;
            #[doc = "16KB."]
            pub const SZ_5: u32 = 0x05;
            #[doc = "32KB."]
            pub const SZ_6: u32 = 0x06;
            #[doc = "64KB."]
            pub const SZ_7: u32 = 0x07;
            #[doc = "128KB."]
            pub const SZ_8: u32 = 0x08;
            #[doc = "256KB."]
            pub const SZ_9: u32 = 0x09;
            #[doc = "512KB."]
            pub const SZ_10: u32 = 0x0a;
            #[doc = "1MB."]
            pub const SZ_11: u32 = 0x0b;
            #[doc = "2MB."]
            pub const SZ_12: u32 = 0x0c;
            #[doc = "4MB."]
            pub const SZ_13: u32 = 0x0d;
            #[doc = "8MB."]
            pub const SZ_14: u32 = 0x0e;
            #[doc = "16MB."]
            pub const SZ_15: u32 = 0x0f;
        }
    }
}
#[doc = "AHBP Control Register"]
pub mod CM7_AHBPCR {
    #[doc = "AHBP enable."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHBP disabled. When disabled all accesses are made to the AXIM interface."]
            pub const EN_0: u32 = 0;
            #[doc = "AHBP enabled."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "AHBP size."]
    pub mod SZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0MB. AHBP disabled."]
            pub const SZ_0: u32 = 0;
            #[doc = "64MB."]
            pub const SZ_1: u32 = 0x01;
            #[doc = "128MB."]
            pub const SZ_2: u32 = 0x02;
            #[doc = "256MB."]
            pub const SZ_3: u32 = 0x03;
            #[doc = "512MB."]
            pub const SZ_4: u32 = 0x04;
        }
    }
}
#[doc = "L1 Cache Control Register"]
pub mod CM7_CACR {
    #[doc = "Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    pub mod SIWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
            pub const SIWT_0: u32 = 0;
            #[doc = "Normal Cacheable shared locations are treated as Write-Through."]
            pub const SIWT_1: u32 = 0x01;
        }
    }
    #[doc = "Enables ECC in the instruction and data cache."]
    pub mod ECCDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables ECC in the instruction and data cache."]
            pub const ECCDIS_0: u32 = 0;
            #[doc = "Disables ECC in the instruction and data cache."]
            pub const ECCDIS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables Force Write-Through in the data cache."]
    pub mod FORCEWT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables Force Write-Through."]
            pub const FORCEWT_0: u32 = 0;
            #[doc = "Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
            pub const FORCEWT_1: u32 = 0x01;
        }
    }
}
#[doc = "AHB Slave Control Register"]
pub mod CM7_AHBSCR {
    #[doc = "AHBS prioritization control."]
    pub mod CTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHBS access priority demoted. This is the reset value."]
            pub const CTL_0: u32 = 0;
            #[doc = "Software access priority demoted."]
            pub const CTL_1: u32 = 0x01;
            #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\\[INITCOUNT\\] value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\\[TPRI\\]."]
            pub const CTL_2: u32 = 0x02;
            #[doc = "AHBSPRI signal has control of access priority."]
            pub const CTL_3: u32 = 0x03;
        }
    }
    #[doc = "Threshold execution priority for AHBS traffic demotion."]
    pub mod TPRI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fairness counter initialization value."]
    pub mod INITCOUNT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Auxiliary Bus Fault Status Register"]
pub mod CM7_ABFSR {
    #[doc = "Asynchronous fault on ITCM interface."]
    pub mod ITCM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous fault on DTCM interface."]
    pub mod DTCM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous fault on AHBP interface."]
    pub mod AHBP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous fault on AXIM interface."]
    pub mod AXIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous fault on EPPB interface."]
    pub mod EPPB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    pub mod AXIMTYPE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OKAY."]
            pub const AXIMTYPE_0: u32 = 0;
            #[doc = "EXOKAY."]
            pub const AXIMTYPE_1: u32 = 0x01;
            #[doc = "SLVERR."]
            pub const AXIMTYPE_2: u32 = 0x02;
            #[doc = "DECERR."]
            pub const AXIMTYPE_3: u32 = 0x03;
        }
    }
}
