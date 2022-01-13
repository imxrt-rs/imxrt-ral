//! Register access layer for imxrt1052

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::aipstz;
pub use super::instances::can;
pub use super::instances::ccm;
pub use super::instances::ccm_analog;
pub use super::instances::cmp;
pub use super::instances::csu;
pub use super::instances::dcdc;
pub use super::instances::dma0;
pub use super::instances::dmamux;
pub use super::instances::ewm;
pub use super::instances::flexio;
pub use super::instances::flexram;
pub use super::instances::flexspi;
pub use super::instances::gpc;
pub use super::instances::gpio;
pub use super::instances::gpt;
pub use super::instances::iomuxc;
pub use super::instances::iomuxc_gpr;
pub use super::instances::iomuxc_snvs;
pub use super::instances::iomuxc_snvs_gpr;
pub use super::instances::kpp;
pub use super::instances::lpuart;
pub use super::instances::ocotp;
pub use super::instances::pgc;
pub use super::instances::pit;
pub use super::instances::pmu;
pub use super::instances::romc;
pub use super::instances::rtwdog;
pub use super::instances::snvs;
pub use super::instances::src;
pub use super::instances::tempmon;
pub use super::instances::tmr;
pub use super::instances::trng;
pub use super::instances::tsc;
pub use super::instances::usb_analog;
pub use super::instances::usbphy;
pub use super::instances::wdog;
pub use super::instances::xtalosc24m;
pub mod csi;
pub mod lcdif;
pub mod pxp;
pub use super::instances::adc_etc;
pub use super::instances::aoi;
pub use super::instances::bee;
pub use super::instances::dcp;
pub use super::instances::enc;
pub use super::instances::enet;
pub use super::instances::lpi2c;
pub use super::instances::lpspi;
pub use super::instances::pwm;
pub use super::instances::sai;
pub use super::instances::semc;
pub use super::instances::spdif;
pub use super::instances::usb;
pub use super::instances::usbnc;
pub use super::instances::usdhc;
pub use super::instances::xbara1;
pub use super::instances::xbarb;

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
    pub USBPHY1: usbphy::Instance<1>,
    pub USBPHY2: usbphy::Instance<2>,
    pub CSU: csu::Instance<0>,
    pub TSC: tsc::Instance<0>,
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
    pub FLEXIO1: flexio::Instance<1>,
    pub FLEXIO2: flexio::Instance<2>,
    pub GPIO1: gpio::Instance<1>,
    pub GPIO5: gpio::Instance<5>,
    pub GPIO2: gpio::Instance<2>,
    pub GPIO3: gpio::Instance<3>,
    pub GPIO4: gpio::Instance<4>,
    pub CAN1: can::Instance<1>,
    pub CAN2: can::Instance<2>,
    pub TMR1: tmr::Instance<1>,
    pub TMR2: tmr::Instance<2>,
    pub TMR3: tmr::Instance<3>,
    pub TMR4: tmr::Instance<4>,
    pub GPT1: gpt::Instance<1>,
    pub GPT2: gpt::Instance<2>,
    pub OCOTP: ocotp::Instance<0>,
    pub IOMUXC: iomuxc::Instance<0>,
    pub KPP: kpp::Instance<0>,
    pub FLEXSPI: flexspi::Instance<0>,
    pub PXP: pxp::Instance<0>,
    pub LCDIF: lcdif::Instance<0>,
    pub CSI: csi::Instance<0>,
    pub USDHC1: usdhc::Instance<1>,
    pub USDHC2: usdhc::Instance<2>,
    pub ENET: enet::Instance<0>,
    pub USB1: usb::Instance<1>,
    pub USB2: usb::Instance<2>,
    pub USBNC1: usbnc::Instance<1>,
    pub USBNC2: usbnc::Instance<2>,
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
    pub AOI1: aoi::Instance<1>,
    pub AOI2: aoi::Instance<2>,
    pub XBARA1: xbara1::Instance<0>,
    pub XBARB2: xbarb::Instance<2>,
    pub XBARB3: xbarb::Instance<3>,
    pub ENC1: enc::Instance<1>,
    pub ENC2: enc::Instance<2>,
    pub ENC3: enc::Instance<3>,
    pub ENC4: enc::Instance<4>,
    pub PWM1: pwm::Instance<1>,
    pub PWM2: pwm::Instance<2>,
    pub PWM3: pwm::Instance<3>,
    pub PWM4: pwm::Instance<4>,
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
            USBPHY1: usbphy::USBPHY1::steal(),
            USBPHY2: usbphy::USBPHY2::steal(),
            CSU: csu::CSU::steal(),
            TSC: tsc::TSC::steal(),
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
            FLEXIO1: flexio::FLEXIO1::steal(),
            FLEXIO2: flexio::FLEXIO2::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO2: gpio::GPIO2::steal(),
            GPIO3: gpio::GPIO3::steal(),
            GPIO4: gpio::GPIO4::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            TMR1: tmr::TMR1::steal(),
            TMR2: tmr::TMR2::steal(),
            TMR3: tmr::TMR3::steal(),
            TMR4: tmr::TMR4::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            KPP: kpp::KPP::steal(),
            FLEXSPI: flexspi::FLEXSPI::steal(),
            PXP: pxp::PXP::steal(),
            LCDIF: lcdif::LCDIF::steal(),
            CSI: csi::CSI::steal(),
            USDHC1: usdhc::USDHC1::steal(),
            USDHC2: usdhc::USDHC2::steal(),
            ENET: enet::ENET::steal(),
            USB1: usb::USB1::steal(),
            USB2: usb::USB2::steal(),
            USBNC1: usbnc::USBNC1::steal(),
            USBNC2: usbnc::USBNC2::steal(),
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
            AOI1: aoi::AOI1::steal(),
            AOI2: aoi::AOI2::steal(),
            XBARA1: xbara1::XBARA1::steal(),
            XBARB2: xbarb::XBARB2::steal(),
            XBARB3: xbarb::XBARB3::steal(),
            ENC1: enc::ENC1::steal(),
            ENC2: enc::ENC2::steal(),
            ENC3: enc::ENC3::steal(),
            ENC4: enc::ENC4::steal(),
            PWM1: pwm::PWM1::steal(),
            PWM2: pwm::PWM2::steal(),
            PWM3: pwm::PWM3::steal(),
            PWM4: pwm::PWM4::steal(),
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
