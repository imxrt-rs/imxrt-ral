//! Register access layer for imxrt1021

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod adc;
pub mod adc_etc;
pub mod aipstz;
pub mod aoi;
pub mod bee;
pub mod can;
pub mod ccm;
pub mod ccm_analog;
pub mod cmp;
pub mod csu;
pub mod dcdc;
pub mod dcp;
pub mod dma0;
pub mod dmamux;
pub mod enc;
pub mod enet;
pub mod ewm;
pub mod flexio1;
pub mod flexram;
pub mod flexspi;
pub mod gpc;
pub mod gpio;
pub mod gpt;
pub mod iomuxc;
pub mod iomuxc_gpr;
pub mod iomuxc_snvs;
pub mod iomuxc_snvs_gpr;
pub mod kpp;
pub mod lpi2c;
pub mod lpspi;
pub mod lpuart;
pub mod ocotp;
pub mod pgc;
pub mod pit;
pub mod pmu;
pub mod pwm;
pub mod romc;
pub mod rtwdog;
pub mod sai;
pub mod semc;
pub mod snvs;
pub mod spdif;
pub mod src;
pub mod tempmon;
pub mod tmr;
pub mod trng;
pub mod usb;
pub mod usb_analog;
pub mod usbnc;
pub mod usbphy;
pub mod usdhc;
pub mod wdog;
pub mod xbara;
pub mod xbarb;
pub mod xtalosc24m;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AIPSTZ1: aipstz::Instance<1>,
    pub AIPSTZ2: aipstz::Instance<2>,
    pub AIPSTZ3: aipstz::Instance<3>,
    pub AIPSTZ4: aipstz::Instance<4>,
    pub DCDC: dcdc::Instance<0>,
    pub PIT: pit::Instance<0>,
    pub CMP1: cmp::Instance<1>,
    pub CMP2: cmp::Instance<2>,
    pub CMP3: cmp::Instance<3>,
    pub CMP4: cmp::Instance<4>,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::Instance<0>,
    pub IOMUXC_SNVS: iomuxc_snvs::Instance<0>,
    pub IOMUXC_GPR: iomuxc_gpr::Instance<0>,
    pub FLEXRAM: flexram::Instance<0>,
    pub EWM: ewm::Instance<0>,
    pub WDOG1: wdog::Instance<1>,
    pub WDOG2: wdog::Instance<2>,
    pub RTWDOG: rtwdog::Instance<0>,
    pub ADC1: adc::Instance<1>,
    pub ADC2: adc::Instance<2>,
    pub TRNG: trng::Instance<0>,
    pub SNVS: snvs::Instance<0>,
    pub CCM_ANALOG: ccm_analog::Instance<0>,
    pub PMU: pmu::Instance<0>,
    pub TEMPMON: tempmon::Instance<0>,
    pub USB_ANALOG: usb_analog::Instance<0>,
    pub XTALOSC24M: xtalosc24m::Instance<0>,
    pub USBPHY: usbphy::Instance<0>,
    pub CSU: csu::Instance<0>,
    pub DMA0: dma0::Instance<0>,
    pub DMAMUX: dmamux::Instance<0>,
    pub GPC: gpc::Instance<0>,
    pub PGC: pgc::Instance<0>,
    pub SRC: src::Instance<0>,
    pub CCM: ccm::Instance<0>,
    pub ROMC: romc::Instance<0>,
    pub LPUART1: lpuart::Instance<1>,
    pub LPUART2: lpuart::Instance<2>,
    pub LPUART3: lpuart::Instance<3>,
    pub LPUART4: lpuart::Instance<4>,
    pub LPUART5: lpuart::Instance<5>,
    pub LPUART6: lpuart::Instance<6>,
    pub LPUART7: lpuart::Instance<7>,
    pub LPUART8: lpuart::Instance<8>,
    pub FLEXIO1: flexio1::Instance<0>,
    pub GPIO1: gpio::Instance<1>,
    pub GPIO5: gpio::Instance<5>,
    pub GPIO2: gpio::Instance<2>,
    pub GPIO3: gpio::Instance<3>,
    pub CAN1: can::Instance<1>,
    pub CAN2: can::Instance<2>,
    pub TMR1: tmr::Instance<1>,
    pub TMR2: tmr::Instance<2>,
    pub GPT1: gpt::Instance<1>,
    pub GPT2: gpt::Instance<2>,
    pub OCOTP: ocotp::Instance<0>,
    pub IOMUXC: iomuxc::Instance<0>,
    pub KPP: kpp::Instance<0>,
    pub FLEXSPI: flexspi::Instance<0>,
    pub USDHC1: usdhc::Instance<1>,
    pub USDHC2: usdhc::Instance<2>,
    pub ENET: enet::Instance<0>,
    pub USB: usb::Instance<0>,
    pub USBNC: usbnc::Instance<0>,
    pub SEMC: semc::Instance<0>,
    pub DCP: dcp::Instance<0>,
    pub SPDIF: spdif::Instance<0>,
    pub SAI1: sai::Instance<1>,
    pub SAI2: sai::Instance<2>,
    pub SAI3: sai::Instance<3>,
    pub LPSPI1: lpspi::Instance<1>,
    pub LPSPI2: lpspi::Instance<2>,
    pub LPSPI3: lpspi::Instance<3>,
    pub LPSPI4: lpspi::Instance<4>,
    pub ADC_ETC: adc_etc::Instance<0>,
    pub AOI: aoi::Instance<0>,
    pub XBARA: xbara::Instance<0>,
    pub XBARB: xbarb::Instance<0>,
    pub ENC1: enc::Instance<1>,
    pub ENC2: enc::Instance<2>,
    pub PWM1: pwm::Instance<1>,
    pub PWM2: pwm::Instance<2>,
    pub BEE: bee::Instance<0>,
    pub LPI2C1: lpi2c::Instance<1>,
    pub LPI2C2: lpi2c::Instance<2>,
    pub LPI2C3: lpi2c::Instance<3>,
    pub LPI2C4: lpi2c::Instance<4>,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            AIPSTZ1: aipstz::AIPSTZ1::steal(),
            AIPSTZ2: aipstz::AIPSTZ2::steal(),
            AIPSTZ3: aipstz::AIPSTZ3::steal(),
            AIPSTZ4: aipstz::AIPSTZ4::steal(),
            DCDC: dcdc::DCDC::steal(),
            PIT: pit::PIT::steal(),
            CMP1: cmp::CMP1::steal(),
            CMP2: cmp::CMP2::steal(),
            CMP3: cmp::CMP3::steal(),
            CMP4: cmp::CMP4::steal(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::steal(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::steal(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::steal(),
            FLEXRAM: flexram::FLEXRAM::steal(),
            EWM: ewm::EWM::steal(),
            WDOG1: wdog::WDOG1::steal(),
            WDOG2: wdog::WDOG2::steal(),
            RTWDOG: rtwdog::RTWDOG::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            TRNG: trng::TRNG::steal(),
            SNVS: snvs::SNVS::steal(),
            CCM_ANALOG: ccm_analog::CCM_ANALOG::steal(),
            PMU: pmu::PMU::steal(),
            TEMPMON: tempmon::TEMPMON::steal(),
            USB_ANALOG: usb_analog::USB_ANALOG::steal(),
            XTALOSC24M: xtalosc24m::XTALOSC24M::steal(),
            USBPHY: usbphy::USBPHY::steal(),
            CSU: csu::CSU::steal(),
            DMA0: dma0::DMA0::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPC: gpc::GPC::steal(),
            PGC: pgc::PGC::steal(),
            SRC: src::SRC::steal(),
            CCM: ccm::CCM::steal(),
            ROMC: romc::ROMC::steal(),
            LPUART1: lpuart::LPUART1::steal(),
            LPUART2: lpuart::LPUART2::steal(),
            LPUART3: lpuart::LPUART3::steal(),
            LPUART4: lpuart::LPUART4::steal(),
            LPUART5: lpuart::LPUART5::steal(),
            LPUART6: lpuart::LPUART6::steal(),
            LPUART7: lpuart::LPUART7::steal(),
            LPUART8: lpuart::LPUART8::steal(),
            FLEXIO1: flexio1::FLEXIO1::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO2: gpio::GPIO2::steal(),
            GPIO3: gpio::GPIO3::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            TMR1: tmr::TMR1::steal(),
            TMR2: tmr::TMR2::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            KPP: kpp::KPP::steal(),
            FLEXSPI: flexspi::FLEXSPI::steal(),
            USDHC1: usdhc::USDHC1::steal(),
            USDHC2: usdhc::USDHC2::steal(),
            ENET: enet::ENET::steal(),
            USB: usb::USB::steal(),
            USBNC: usbnc::USBNC::steal(),
            SEMC: semc::SEMC::steal(),
            DCP: dcp::DCP::steal(),
            SPDIF: spdif::SPDIF::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SAI3: sai::SAI3::steal(),
            LPSPI1: lpspi::LPSPI1::steal(),
            LPSPI2: lpspi::LPSPI2::steal(),
            LPSPI3: lpspi::LPSPI3::steal(),
            LPSPI4: lpspi::LPSPI4::steal(),
            ADC_ETC: adc_etc::ADC_ETC::steal(),
            AOI: aoi::AOI::steal(),
            XBARA: xbara::XBARA::steal(),
            XBARB: xbarb::XBARB::steal(),
            ENC1: enc::ENC1::steal(),
            ENC2: enc::ENC2::steal(),
            PWM1: pwm::PWM1::steal(),
            PWM2: pwm::PWM2::steal(),
            BEE: bee::BEE::steal(),
            LPI2C1: lpi2c::LPI2C1::steal(),
            LPI2C2: lpi2c::LPI2C2::steal(),
            LPI2C3: lpi2c::LPI2C3::steal(),
            LPI2C4: lpi2c::LPI2C4::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
