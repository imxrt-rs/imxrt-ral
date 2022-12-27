#[doc = "CSI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CSI Control Register 1"]
    pub CR1: crate::RWRegister<u32>,
    #[doc = "CSI Control Register 2"]
    pub CR2: crate::RWRegister<u32>,
    #[doc = "CSI Control Register 3"]
    pub CR3: crate::RWRegister<u32>,
    #[doc = "CSI Statistic FIFO Register"]
    pub STATFIFO: crate::RORegister<u32>,
    #[doc = "CSI RX FIFO Register"]
    pub RFIFO: crate::RORegister<u32>,
    #[doc = "CSI RX Count Register"]
    pub RXCNT: crate::RWRegister<u32>,
    #[doc = "CSI Status Register"]
    pub SR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "CSI DMA Start Address Register - for STATFIFO"]
    pub DMASA_STATFIFO: crate::RWRegister<u32>,
    #[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
    pub DMATS_STATFIFO: crate::RWRegister<u32>,
    #[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
    pub DMASA_FB1: crate::RWRegister<u32>,
    #[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
    pub DMASA_FB2: crate::RWRegister<u32>,
    #[doc = "CSI Frame Buffer Parameter Register"]
    pub FBUF_PARA: crate::RWRegister<u32>,
    #[doc = "CSI Image Parameter Register"]
    pub IMAG_PARA: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "CSI Control Register 18"]
    pub CR18: crate::RWRegister<u32>,
    #[doc = "CSI Control Register 19"]
    pub CR19: crate::RWRegister<u32>,
    #[doc = "CSI Control Register 20"]
    pub CR20: crate::RWRegister<u32>,
    #[doc = "CSI Control Register"]
    pub CR: [crate::RWRegister<u32>; 256usize],
}
#[doc = "CSI Control Register 1"]
pub mod CR1 {
    #[doc = "Pixel Bit"]
    pub mod PIXEL_BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit data for each pixel"]
            pub const PIXEL_BIT_0: u32 = 0;
            #[doc = "10-bit data for each pixel"]
            pub const PIXEL_BIT_1: u32 = 0x01;
        }
    }
    #[doc = "Valid Pixel Clock Edge Select"]
    pub mod REDGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pixel data is latched at the falling edge of CSI_PIXCLK"]
            pub const REDGE_0: u32 = 0;
            #[doc = "Pixel data is latched at the rising edge of CSI_PIXCLK"]
            pub const REDGE_1: u32 = 0x01;
        }
    }
    #[doc = "Invert Pixel Clock Input"]
    pub mod INV_PCLK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CSI_PIXCLK is directly applied to internal circuitry"]
            pub const INV_PCLK_0: u32 = 0;
            #[doc = "CSI_PIXCLK is inverted before applied to internal circuitry"]
            pub const INV_PCLK_1: u32 = 0x01;
        }
    }
    #[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    pub mod INV_DATA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CSI_D\\[7:0\\] data lines are directly applied to internal circuitry"]
            pub const INV_DATA_0: u32 = 0;
            #[doc = "CSI_D\\[7:0\\] data lines are inverted before applied to internal circuitry"]
            pub const INV_DATA_1: u32 = 0x01;
        }
    }
    #[doc = "Gated Clock Mode Enable"]
    pub mod GCLK_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
            pub const GCLK_MODE_0: u32 = 0;
            #[doc = "Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
            pub const GCLK_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Asynchronous RXFIFO Clear"]
    pub mod CLR_RXFIFO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous STATFIFO Clear"]
    pub mod CLR_STATFIFO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Packing Direction"]
    pub mod PACK_DIR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
            pub const PACK_DIR_0: u32 = 0;
            #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
            pub const PACK_DIR_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Clear Control"]
    pub mod FCC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous FIFO clear is selected."]
            pub const FCC_0: u32 = 0;
            #[doc = "Synchronous FIFO clear is selected."]
            pub const FCC_1: u32 = 0x01;
        }
    }
    #[doc = "BT.656 Interface Enable. This bit selects the type of interface used."]
    pub mod CCIR_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Traditional interface is selected. Timing interface logic is used to latch data."]
            pub const CCIR_EN_0: u32 = 0;
            #[doc = "CCIR656 interface is selected."]
            pub const CCIR_EN_1: u32 = 0x01;
        }
    }
    #[doc = "HSYNC Polarity Select"]
    pub mod HSYNC_POL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HSYNC is active low"]
            pub const HSYNC_POL_0: u32 = 0;
            #[doc = "HSYNC is active high"]
            pub const HSYNC_POL_1: u32 = 0x01;
        }
    }
    #[doc = "Histogram Interrupt Enable"]
    pub mod HISTOGRAM_CALC_DONE_IE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Histogram done interrupt disable"]
            pub const HISTOGRAM_CALC_DONE_IE_0: u32 = 0;
            #[doc = "Histogram done interrupt enable"]
            pub const HISTOGRAM_CALC_DONE_IE_1: u32 = 0x01;
        }
    }
    #[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    pub mod SOF_INTEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOF interrupt disable"]
            pub const SOF_INTEN_0: u32 = 0;
            #[doc = "SOF interrupt enable"]
            pub const SOF_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    pub mod SOF_POL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOF interrupt is generated on SOF falling edge"]
            pub const SOF_POL_0: u32 = 0;
            #[doc = "SOF interrupt is generated on SOF rising edge"]
            pub const SOF_POL_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    pub mod RXFF_INTEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RxFIFO full interrupt disable"]
            pub const RXFF_INTEN_0: u32 = 0;
            #[doc = "RxFIFO full interrupt enable"]
            pub const RXFF_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    pub mod FB1_DMA_DONE_INTEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Buffer1 DMA Transfer Done interrupt disable"]
            pub const FB1_DMA_DONE_INTEN_0: u32 = 0;
            #[doc = "Frame Buffer1 DMA Transfer Done interrupt enable"]
            pub const FB1_DMA_DONE_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    pub mod FB2_DMA_DONE_INTEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Buffer2 DMA Transfer Done interrupt disable"]
            pub const FB2_DMA_DONE_INTEN_0: u32 = 0;
            #[doc = "Frame Buffer2 DMA Transfer Done interrupt enable"]
            pub const FB2_DMA_DONE_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    pub mod STATFF_INTEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STATFIFO full interrupt disable"]
            pub const STATFF_INTEN_0: u32 = 0;
            #[doc = "STATFIFO full interrupt enable"]
            pub const STATFF_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "STATFIFO DMA Transfer Done Interrupt Enable"]
    pub mod SFF_DMA_DONE_INTEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STATFIFO DMA Transfer Done interrupt disable"]
            pub const SFF_DMA_DONE_INTEN_0: u32 = 0;
            #[doc = "STATFIFO DMA Transfer Done interrupt enable"]
            pub const SFF_DMA_DONE_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    pub mod RF_OR_INTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RxFIFO overrun interrupt is disabled"]
            pub const RF_OR_INTEN_0: u32 = 0;
            #[doc = "RxFIFO overrun interrupt is enabled"]
            pub const RF_OR_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    pub mod SF_OR_INTEN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STATFIFO overrun interrupt is disabled"]
            pub const SF_OR_INTEN_0: u32 = 0;
            #[doc = "STATFIFO overrun interrupt is enabled"]
            pub const SF_OR_INTEN_1: u32 = 0x01;
        }
    }
    #[doc = "Change Of Image Field (COF) Interrupt Enable"]
    pub mod COF_INT_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COF interrupt is disabled"]
            pub const COF_INT_EN_0: u32 = 0;
            #[doc = "COF interrupt is enabled"]
            pub const COF_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Video mode select. This bit controls the video mode in BT.656 mode and TV decoder input."]
    pub mod VIDEO_MODE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Progressive mode is selected"]
            pub const VIDEO_MODE_0: u32 = 0;
            #[doc = "Interlace mode is selected"]
            pub const VIDEO_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    pub mod EOF_INT_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EOF interrupt is disabled."]
            pub const EOF_INT_EN_0: u32 = 0;
            #[doc = "EOF interrupt is generated when RX count value is reached."]
            pub const EOF_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "External VSYNC Enable"]
    pub mod EXT_VSYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal VSYNC mode"]
            pub const EXT_VSYNC_0: u32 = 0;
            #[doc = "External VSYNC mode"]
            pub const EXT_VSYNC_1: u32 = 0x01;
        }
    }
    #[doc = "SWAP 16-Bit Enable"]
    pub mod SWAP16_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable swapping"]
            pub const SWAP16_EN_0: u32 = 0;
            #[doc = "Enable swapping"]
            pub const SWAP16_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "CSI Control Register 2"]
pub mod CR2 {
    #[doc = "Horizontal Skip Count"]
    pub mod HSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_0: u32 = 0;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_1: u32 = 0x01;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_2: u32 = 0x02;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_3: u32 = 0x03;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_4: u32 = 0x04;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_5: u32 = 0x05;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_6: u32 = 0x06;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_7: u32 = 0x07;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_8: u32 = 0x08;
            #[doc = "Number of pixels to skip minus 1"]
            pub const HSC_9: u32 = 0x09;
        }
    }
    #[doc = "Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    pub mod VSC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_0: u32 = 0;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_1: u32 = 0x01;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_2: u32 = 0x02;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_3: u32 = 0x03;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_4: u32 = 0x04;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_5: u32 = 0x05;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_6: u32 = 0x06;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_7: u32 = 0x07;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_8: u32 = 0x08;
            #[doc = "Number of rows to skip minus 1"]
            pub const VSC_9: u32 = 0x09;
        }
    }
    #[doc = "Live View Resolution Mode. Selects the grid size used for live view resolution."]
    pub mod LVRM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "512 x 384"]
            pub const LVRM_0: u32 = 0;
            #[doc = "448 x 336"]
            pub const LVRM_1: u32 = 0x01;
            #[doc = "384 x 288"]
            pub const LVRM_2: u32 = 0x02;
            #[doc = "384 x 256"]
            pub const LVRM_3: u32 = 0x03;
            #[doc = "320 x 240"]
            pub const LVRM_4: u32 = 0x04;
            #[doc = "288 x 216"]
            pub const LVRM_5: u32 = 0x05;
            #[doc = "400 x 300"]
            pub const LVRM_6: u32 = 0x06;
        }
    }
    #[doc = "Bayer Tile Start. Controls the Bayer pattern starting point."]
    pub mod BTS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GR"]
            pub const BTS_0: u32 = 0;
            #[doc = "RG"]
            pub const BTS_1: u32 = 0x01;
            #[doc = "BG"]
            pub const BTS_2: u32 = 0x02;
            #[doc = "GB"]
            pub const BTS_3: u32 = 0x03;
        }
    }
    #[doc = "Skip Count Enable"]
    pub mod SCE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Skip count disable"]
            pub const SCE_0: u32 = 0;
            #[doc = "Skip count enable"]
            pub const SCE_1: u32 = 0x01;
        }
    }
    #[doc = "Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    pub mod AFS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abs Diff on consecutive green pixels"]
            pub const AFS_0: u32 = 0;
            #[doc = "Abs Diff on every third green pixels"]
            pub const AFS_1: u32 = 0x01;
            #[doc = "Abs Diff on every four green pixels"]
            pub const AFS_2: u32 = 0x02;
        }
    }
    #[doc = "Double Resolution Mode. Controls size of statistics grid."]
    pub mod DRM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stats grid of 8 x 6"]
            pub const DRM_0: u32 = 0;
            #[doc = "Stats grid of 8 x 12"]
            pub const DRM_1: u32 = 0x01;
        }
    }
    #[doc = "Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    pub mod DMA_BURST_TYPE_SFF {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INCR8"]
            pub const DMA_BURST_TYPE_SFF_0: u32 = 0;
            #[doc = "INCR4"]
            pub const DMA_BURST_TYPE_SFF_1: u32 = 0x01;
            #[doc = "INCR16"]
            pub const DMA_BURST_TYPE_SFF_3: u32 = 0x03;
        }
    }
    #[doc = "Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    pub mod DMA_BURST_TYPE_RFF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INCR8"]
            pub const DMA_BURST_TYPE_RFF_0: u32 = 0;
            #[doc = "INCR4"]
            pub const DMA_BURST_TYPE_RFF_1: u32 = 0x01;
            #[doc = "INCR16"]
            pub const DMA_BURST_TYPE_RFF_3: u32 = 0x03;
        }
    }
}
#[doc = "CSI Control Register 3"]
pub mod CR3 {
    #[doc = "Automatic Error Correction Enable"]
    pub mod ECC_AUTO_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auto Error correction is disabled."]
            pub const ECC_AUTO_EN_0: u32 = 0;
            #[doc = "Auto Error correction is enabled."]
            pub const ECC_AUTO_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Error Detection Interrupt Enable"]
    pub mod ECC_INT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
            pub const ECC_INT_EN_0: u32 = 0;
            #[doc = "Interrupt is generated when error is detected."]
            pub const ECC_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Dummy Zero Packing Enable"]
    pub mod ZERO_PACK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero packing disabled"]
            pub const ZERO_PACK_EN_0: u32 = 0;
            #[doc = "Zero packing enabled"]
            pub const ZERO_PACK_EN_1: u32 = 0x01;
        }
    }
    #[doc = "16-bit Sensor Mode"]
    pub mod SENSOR_16BITS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only one 8-bit sensor is connected."]
            pub const SENSOR_16BITS_0: u32 = 0;
            #[doc = "One 16-bit sensor is connected."]
            pub const SENSOR_16BITS_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO Full Level"]
    pub mod RXFF_LEVEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4 Double words"]
            pub const RXFF_LEVEL_0: u32 = 0;
            #[doc = "8 Double words"]
            pub const RXFF_LEVEL_1: u32 = 0x01;
            #[doc = "16 Double words"]
            pub const RXFF_LEVEL_2: u32 = 0x02;
            #[doc = "24 Double words"]
            pub const RXFF_LEVEL_3: u32 = 0x03;
            #[doc = "32 Double words"]
            pub const RXFF_LEVEL_4: u32 = 0x04;
            #[doc = "48 Double words"]
            pub const RXFF_LEVEL_5: u32 = 0x05;
            #[doc = "64 Double words"]
            pub const RXFF_LEVEL_6: u32 = 0x06;
            #[doc = "96 Double words"]
            pub const RXFF_LEVEL_7: u32 = 0x07;
        }
    }
    #[doc = "Hresponse Error Enable. This bit enables the hresponse (AHB protocol standard) error interrupt."]
    pub mod HRESP_ERR_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable hresponse error interrupt"]
            pub const HRESP_ERR_EN_0: u32 = 0;
            #[doc = "Enable hresponse error interrupt"]
            pub const HRESP_ERR_EN_1: u32 = 0x01;
        }
    }
    #[doc = "STATFIFO Full Level"]
    pub mod STATFF_LEVEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4 Double words"]
            pub const STATFF_LEVEL_0: u32 = 0;
            #[doc = "8 Double words"]
            pub const STATFF_LEVEL_1: u32 = 0x01;
            #[doc = "12 Double words"]
            pub const STATFF_LEVEL_2: u32 = 0x02;
            #[doc = "16 Double words"]
            pub const STATFF_LEVEL_3: u32 = 0x03;
            #[doc = "24 Double words"]
            pub const STATFF_LEVEL_4: u32 = 0x04;
            #[doc = "32 Double words"]
            pub const STATFF_LEVEL_5: u32 = 0x05;
            #[doc = "48 Double words"]
            pub const STATFF_LEVEL_6: u32 = 0x06;
            #[doc = "64 Double words"]
            pub const STATFF_LEVEL_7: u32 = 0x07;
        }
    }
    #[doc = "DMA Request Enable for STATFIFO"]
    pub mod DMA_REQ_EN_SFF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the dma request"]
            pub const DMA_REQ_EN_SFF_0: u32 = 0;
            #[doc = "Enable the dma request"]
            pub const DMA_REQ_EN_SFF_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Request Enable for RxFIFO"]
    pub mod DMA_REQ_EN_RFF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the dma request"]
            pub const DMA_REQ_EN_RFF_0: u32 = 0;
            #[doc = "Enable the dma request"]
            pub const DMA_REQ_EN_RFF_1: u32 = 0x01;
        }
    }
    #[doc = "Reflash DMA Controller for STATFIFO"]
    pub mod DMA_REFLASH_SFF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reflashing"]
            pub const DMA_REFLASH_SFF_0: u32 = 0;
            #[doc = "Reflash the embedded DMA controller"]
            pub const DMA_REFLASH_SFF_1: u32 = 0x01;
        }
    }
    #[doc = "Reflash DMA Controller for RxFIFO"]
    pub mod DMA_REFLASH_RFF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reflashing"]
            pub const DMA_REFLASH_RFF_0: u32 = 0;
            #[doc = "Reflash the embedded DMA controller"]
            pub const DMA_REFLASH_RFF_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    pub mod FRMCNT_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not reset"]
            pub const FRMCNT_RST_0: u32 = 0;
            #[doc = "Reset frame counter immediately"]
            pub const FRMCNT_RST_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Counter"]
    pub mod FRMCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Statistic FIFO Register"]
pub mod STATFIFO {
    #[doc = "Static data from sensor"]
    pub mod STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI RX FIFO Register"]
pub mod RFIFO {
    #[doc = "Received image data"]
    pub mod IMAGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI RX Count Register"]
pub mod RXCNT {
    #[doc = "RxFIFO Count"]
    pub mod RXCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Status Register"]
pub mod SR {
    #[doc = "RXFIFO Data Ready"]
    pub mod DRDY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No data (word) is ready"]
            pub const DRDY_0: u32 = 0;
            #[doc = "At least 1 datum (word) is ready in RXFIFO."]
            pub const DRDY_1: u32 = 0x01;
        }
    }
    #[doc = "BT"]
    pub mod ECC_INT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error detected"]
            pub const ECC_INT_0: u32 = 0;
            #[doc = "Error is detected in CCIR coding"]
            pub const ECC_INT_1: u32 = 0x01;
        }
    }
    #[doc = "no description available"]
    pub mod HISTOGRAM_CALC_DONE_INT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Histogram calculation is not finished"]
            pub const HISTOGRAM_CALC_DONE_INT_0: u32 = 0;
            #[doc = "Histogram calculation is done and driver can access the PIXEL_COUNTERS(CSI_CSICR21~CSI_CSICR276) to get the gray level"]
            pub const HISTOGRAM_CALC_DONE_INT_1: u32 = 0x01;
        }
    }
    #[doc = "Hresponse Error Interrupt Status"]
    pub mod HRESP_ERR_INT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No hresponse error."]
            pub const HRESP_ERR_INT_0: u32 = 0;
            #[doc = "Hresponse error is detected."]
            pub const HRESP_ERR_INT_1: u32 = 0x01;
        }
    }
    #[doc = "Change Of Field Interrupt Status"]
    pub mod COF_INT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Video field has no change."]
            pub const COF_INT_0: u32 = 0;
            #[doc = "Change of video field is detected."]
            pub const COF_INT_1: u32 = 0x01;
        }
    }
    #[doc = "BT"]
    pub mod F1_INT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field 1 of video is not detected."]
            pub const F1_INT_0: u32 = 0;
            #[doc = "Field 1 of video is about to start."]
            pub const F1_INT_1: u32 = 0x01;
        }
    }
    #[doc = "BT"]
    pub mod F2_INT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field 2 of video is not detected"]
            pub const F2_INT_0: u32 = 0;
            #[doc = "Field 2 of video is about to start"]
            pub const F2_INT_1: u32 = 0x01;
        }
    }
    #[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    pub mod SOF_INT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOF is not detected."]
            pub const SOF_INT_0: u32 = 0;
            #[doc = "SOF is detected."]
            pub const SOF_INT_1: u32 = 0x01;
        }
    }
    #[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    pub mod EOF_INT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EOF is not detected."]
            pub const EOF_INT_0: u32 = 0;
            #[doc = "EOF is detected."]
            pub const EOF_INT_1: u32 = 0x01;
        }
    }
    #[doc = "RXFIFO Full Interrupt Status"]
    pub mod RXFF_INT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RxFIFO is not full."]
            pub const RXFF_INT_0: u32 = 0;
            #[doc = "RxFIFO is full."]
            pub const RXFF_INT_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Transfer Done in Frame Buffer1"]
    pub mod DMA_TSF_DONE_FB1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA transfer is not completed."]
            pub const DMA_TSF_DONE_FB1_0: u32 = 0;
            #[doc = "DMA transfer is completed."]
            pub const DMA_TSF_DONE_FB1_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Transfer Done in Frame Buffer2"]
    pub mod DMA_TSF_DONE_FB2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA transfer is not completed."]
            pub const DMA_TSF_DONE_FB2_0: u32 = 0;
            #[doc = "DMA transfer is completed."]
            pub const DMA_TSF_DONE_FB2_1: u32 = 0x01;
        }
    }
    #[doc = "STATFIFO Full Interrupt Status"]
    pub mod STATFF_INT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STATFIFO is not full."]
            pub const STATFF_INT_0: u32 = 0;
            #[doc = "STATFIFO is full."]
            pub const STATFF_INT_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Transfer Done from StatFIFO"]
    pub mod DMA_TSF_DONE_SFF {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA transfer is not completed."]
            pub const DMA_TSF_DONE_SFF_0: u32 = 0;
            #[doc = "DMA transfer is completed."]
            pub const DMA_TSF_DONE_SFF_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO Overrun Interrupt Status"]
    pub mod RF_OR_INT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RXFIFO has not overflowed."]
            pub const RF_OR_INT_0: u32 = 0;
            #[doc = "RXFIFO has overflowed."]
            pub const RF_OR_INT_1: u32 = 0x01;
        }
    }
    #[doc = "STATFIFO Overrun Interrupt Status"]
    pub mod SF_OR_INT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STATFIFO has not overflowed."]
            pub const SF_OR_INT_0: u32 = 0;
            #[doc = "STATFIFO has overflowed."]
            pub const SF_OR_INT_1: u32 = 0x01;
        }
    }
    #[doc = "When DMA field 1 is complete, this bit will be set to 1(clear by writing 1)."]
    pub mod DMA_FIELD1_DONE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    pub mod DMA_FIELD0_DONE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    pub mod BASEADDR_CHHANGE_ERROR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI DMA Start Address Register - for STATFIFO"]
pub mod DMASA_STATFIFO {
    #[doc = "DMA Start Address for STATFIFO"]
    pub mod DMA_START_ADDR_SFF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
pub mod DMATS_STATFIFO {
    #[doc = "DMA Transfer Size for STATFIFO"]
    pub mod DMA_TSF_SIZE_SFF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
pub mod DMASA_FB1 {
    #[doc = "DMA Start Address in Frame Buffer1"]
    pub mod DMA_START_ADDR_FB1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
pub mod DMASA_FB2 {
    #[doc = "DMA Start Address in Frame Buffer2"]
    pub mod DMA_START_ADDR_FB2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Frame Buffer Parameter Register"]
pub mod FBUF_PARA {
    #[doc = "Frame Buffer Parameter"]
    pub mod FBUF_STRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEINTERLACE_STRIDE is only used in the deinterlace mode"]
    pub mod DEINTERLACE_STRIDE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Image Parameter Register"]
pub mod IMAG_PARA {
    #[doc = "Image Height. Indicates how many pixels in a column of the image from the sensor."]
    pub mod IMAGE_HEIGHT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the number of active pixel cycles per line"]
    pub mod IMAGE_WIDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Control Register 18"]
pub mod CR18 {
    #[doc = "This bit is used to select NTSC/PAL mode When input is TVDECODER or standard BT.656 video."]
    pub mod NTSC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PAL"]
            pub const NTSC_EN_0: u32 = 0;
            #[doc = "NTSC"]
            pub const NTSC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "When input is from TV decoder, this bit is enabled."]
    pub mod TVDECODER_IN_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is used to select the output method When input is TVDECODER or standard BT.656 video."]
    pub mod DEINTERLACE_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deinterlace disabled"]
            pub const DEINTERLACE_EN_0: u32 = 0;
            #[doc = "Deinterlace enabled"]
            pub const DEINTERLACE_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Enable bit for Parallel RGB888/YUV444 24bit input"]
    pub mod PARALLEL24_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input is disabled"]
            pub const PARALLEL24_EN_0: u32 = 0;
            #[doc = "Input is enabled"]
            pub const PARALLEL24_EN_1: u32 = 0x01;
        }
    }
    #[doc = "When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than automatically by DMA completed"]
    pub mod BASEADDR_SWITCH_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    pub mod BASEADDR_SWITCH_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Switching base address at the edge of the vsync"]
            pub const BASEADDR_SWITCH_SEL_0: u32 = 0;
            #[doc = "Switching base address at the edge of the first data of each frame"]
            pub const BASEADDR_SWITCH_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "In interlace mode, field 0 means interrupt enabled."]
    pub mod FIELD0_DONE_IE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const FIELD0_DONE_IE_0: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const FIELD0_DONE_IE_1: u32 = 0x01;
        }
    }
    #[doc = "When in interlace mode, field 1 done interrupt enable."]
    pub mod DMA_FIELD1_DONE_IE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DMA_FIELD1_DONE_IE_0: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const DMA_FIELD1_DONE_IE_1: u32 = 0x01;
        }
    }
    #[doc = "Choosing the last DMA request condition"]
    pub mod LAST_DMA_REQ_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "fifo_full_level"]
            pub const LAST_DMA_REQ_SEL_0: u32 = 0;
            #[doc = "hburst_length"]
            pub const LAST_DMA_REQ_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Base address change error interrupt enable signal."]
    pub mod BASEADDR_CHANGE_ERROR_IE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const BASEADDR_CHANGE_ERROR_IE_0: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const BASEADDR_CHANGE_ERROR_IE_1: u32 = 0x01;
        }
    }
    #[doc = "Output is 32-bit format."]
    pub mod RGB888A_FORMAT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "{8'h0, data\\[23:0\\]}"]
            pub const RGB888A_FORMAT_SEL_0: u32 = 0;
            #[doc = "{data\\[23:0\\], 8'h0}"]
            pub const RGB888A_FORMAT_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Hprot value in AHB bus protocol."]
    pub mod AHB_HPROT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits used to choose the method to mask the CSI input."]
    pub mod MASK_OPTION {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1."]
            pub const MASK_OPTION_0: u32 = 0;
            #[doc = "Writing to memory when CSI_ENABLE is 1."]
            pub const MASK_OPTION_1: u32 = 0x01;
            #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
            pub const MASK_OPTION_2: u32 = 0x02;
            #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
            pub const MASK_OPTION_3: u32 = 0x03;
        }
    }
    #[doc = "Double component per clock cycle in YUV422 formats."]
    pub mod MIPI_DOUBLE_CMPNT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single component per clock cycle (half pixel per clock cycle)"]
            pub const MIPI_DOUBLE_CMPNT_0: u32 = 0;
            #[doc = "Double component per clock cycle (a pixel per clock cycle)"]
            pub const MIPI_DOUBLE_CMPNT_1: u32 = 0x01;
        }
    }
    #[doc = "It only works in MIPI CSI YUV422 double component mode."]
    pub mod MIPI_YU_SWAP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "no description available"]
    pub mod DATA_FROM_MIPI {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data from parallel sensor"]
            pub const DATA_FROM_MIPI_0: u32 = 0;
            #[doc = "Data from MIPI"]
            pub const DATA_FROM_MIPI_1: u32 = 0x01;
        }
    }
    #[doc = "When the line width are not the multiple of the burst length, assert this bit."]
    pub mod LINE_STRIDE_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Image Data Format"]
    pub mod MIPI_DATA_FORMAT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSI global enable signal"]
    pub mod CSI_ENABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Control Register 19"]
pub mod CR19 {
    #[doc = "This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it"]
    pub mod DMA_RFIFO_HIGHEST_FIFO_LEVEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSI Control Register 20"]
pub mod CR20 {
    #[doc = "THRESHOLD used for binary function. When data value > THRESHOLD, output will be 1 Else will be 0."]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "no description available"]
    pub mod BINARY_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is Y8 format(8 bits each pixel)"]
            pub const BINARY_EN_0: u32 = 0;
            #[doc = "Output is Y1 format(1 bit each pixel)"]
            pub const BINARY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "no description available"]
    pub mod QR_DATA_FORMAT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "YU YV one cycle per 1 pixel input"]
            pub const QR_DATA_FORMAT_0: u32 = 0;
            #[doc = "UY VY one cycle per1 pixel input"]
            pub const QR_DATA_FORMAT_1: u32 = 0x01;
            #[doc = "Y U Y V two cycles per 1 pixel input"]
            pub const QR_DATA_FORMAT_2: u32 = 0x02;
            #[doc = "U Y V Y two cycles per 1 pixel input"]
            pub const QR_DATA_FORMAT_3: u32 = 0x03;
            #[doc = "YUV one cycle per 1 pixel input"]
            pub const QR_DATA_FORMAT_4: u32 = 0x04;
            #[doc = "Y U V three cycles per 1 pixel input"]
            pub const QR_DATA_FORMAT_5: u32 = 0x05;
        }
    }
    #[doc = "no description available"]
    pub mod BIG_END {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The newest (most recent) data will be assigned the lowest position when store to memory."]
            pub const BIG_END_0: u32 = 0;
            #[doc = "The newest (most recent) data will be assigned the highest position when store to memory."]
            pub const BIG_END_1: u32 = 0x01;
        }
    }
    #[doc = "no description available"]
    pub mod _10BIT_NEW_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When input 8bits data, it will use the data\\[9:2\\]"]
            pub const _10BIT_NEW_EN_0: u32 = 0;
            #[doc = "If input is 10bits data, it will use the data\\[7:0\\] (optional)"]
            pub const _10BIT_NEW_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Histogram enable"]
    pub mod HISTOGRAM_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Histogram disable"]
            pub const HISTOGRAM_EN_0: u32 = 0;
            #[doc = "Histogram enable"]
            pub const HISTOGRAM_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Gray scale mode enable"]
    pub mod QRCODE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const QRCODE_EN_0: u32 = 0;
            #[doc = "Gray scale mode"]
            pub const QRCODE_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "CSI Control Register"]
pub mod CR {
    #[doc = "Number of pixels (Y component of the input pixel) equals: 0 (CSICR21) 1 (CSICR22)"]
    pub mod PIXEL_COUNTERS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
