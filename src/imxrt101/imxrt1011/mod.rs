//! Register access layer for imxrt1011

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod adc_etc;
pub mod aipstz;
pub mod aoi;
pub mod dcdc;
pub mod flexspi;
pub mod otfad;
pub mod pit;
pub mod xbara;
pub use super::instances::iomuxc_snvs_gpr;
pub mod flexram;
pub mod iomuxc_gpr;
pub mod iomuxc_snvs;
pub use super::instances::ewm;
pub mod wdog;
pub use super::instances::rtwdog;
pub mod adc;
pub mod trng;
pub use super::instances::ccm_analog;
pub use super::instances::snvs;
pub mod pmu;
pub use super::instances::tempmon;
pub mod usb_analog;
pub use super::instances::csu;
pub use super::instances::usbphy;
pub use super::instances::xtalosc24m;
pub mod dcp;
pub mod dma0;
pub mod dmamux;
pub mod gpc;
pub mod usb;
pub mod usbnc;
pub use super::instances::pgc;
pub mod ccm;
pub mod src;
pub use super::instances::lpuart;
pub use super::instances::romc;
pub mod lpi2c;
pub mod lpspi;
pub use super::instances::flexio1;
pub mod gpio;
pub mod pwm;
pub mod sai;
pub mod spdif;
pub use super::instances::gpt;
pub mod iomuxc;
pub mod ocotp;
pub use super::instances::kpp;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AIPSTZ1: aipstz::Instance<1>,
    pub AIPSTZ2: aipstz::Instance<2>,
    pub DCDC: dcdc::Instance<0>,
    pub PIT: pit::Instance<0>,
    pub ADC_ETC: adc_etc::Instance<0>,
    pub AOI: aoi::Instance<0>,
    pub XBARA: xbara::Instance<0>,
    pub FLEXSPI: flexspi::Instance<0>,
    pub OTFAD: otfad::Instance<0>,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::Instance<0>,
    pub IOMUXC_SNVS: iomuxc_snvs::Instance<0>,
    pub IOMUXC_GPR: iomuxc_gpr::Instance<0>,
    pub FLEXRAM: flexram::Instance<0>,
    pub EWM: ewm::Instance<0>,
    pub WDOG1: wdog::Instance<1>,
    pub WDOG2: wdog::Instance<2>,
    pub RTWDOG: rtwdog::Instance<0>,
    pub ADC: adc::Instance<0>,
    pub TRNG: trng::Instance<0>,
    pub SNVS: snvs::Instance<0>,
    pub CCM_ANALOG: ccm_analog::Instance<0>,
    pub PMU: pmu::Instance<0>,
    pub TEMPMON: tempmon::Instance<0>,
    pub USB_ANALOG: usb_analog::Instance<0>,
    pub XTALOSC24M: xtalosc24m::Instance<0>,
    pub USBPHY: usbphy::Instance<0>,
    pub CSU: csu::Instance<0>,
    pub USB: usb::Instance<0>,
    pub USBNC: usbnc::Instance<0>,
    pub DMA0: dma0::Instance<0>,
    pub DMAMUX: dmamux::Instance<0>,
    pub DCP: dcp::Instance<0>,
    pub GPC: gpc::Instance<0>,
    pub PGC: pgc::Instance<0>,
    pub SRC: src::Instance<0>,
    pub CCM: ccm::Instance<0>,
    pub ROMC: romc::Instance<0>,
    pub LPUART1: lpuart::Instance<1>,
    pub LPUART2: lpuart::Instance<2>,
    pub LPUART3: lpuart::Instance<3>,
    pub LPUART4: lpuart::Instance<4>,
    pub LPSPI1: lpspi::Instance<1>,
    pub LPSPI2: lpspi::Instance<2>,
    pub LPI2C1: lpi2c::Instance<1>,
    pub LPI2C2: lpi2c::Instance<2>,
    pub FLEXIO1: flexio1::Instance<0>,
    pub GPIO1: gpio::Instance<1>,
    pub GPIO5: gpio::Instance<5>,
    pub GPIO2: gpio::Instance<2>,
    pub PWM: pwm::Instance<0>,
    pub SPDIF: spdif::Instance<0>,
    pub SAI1: sai::Instance<1>,
    pub SAI3: sai::Instance<3>,
    pub GPT1: gpt::Instance<1>,
    pub GPT2: gpt::Instance<2>,
    pub OCOTP: ocotp::Instance<0>,
    pub IOMUXC: iomuxc::Instance<0>,
    pub KPP: kpp::Instance<0>,
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
            DCDC: dcdc::DCDC::steal(),
            PIT: pit::PIT::steal(),
            ADC_ETC: adc_etc::ADC_ETC::steal(),
            AOI: aoi::AOI::steal(),
            XBARA: xbara::XBARA::steal(),
            FLEXSPI: flexspi::FLEXSPI::steal(),
            OTFAD: otfad::OTFAD::steal(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::steal(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::steal(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::steal(),
            FLEXRAM: flexram::FLEXRAM::steal(),
            EWM: ewm::EWM::steal(),
            WDOG1: wdog::WDOG1::steal(),
            WDOG2: wdog::WDOG2::steal(),
            RTWDOG: rtwdog::RTWDOG::steal(),
            ADC: adc::ADC::steal(),
            TRNG: trng::TRNG::steal(),
            SNVS: snvs::SNVS::steal(),
            CCM_ANALOG: ccm_analog::CCM_ANALOG::steal(),
            PMU: pmu::PMU::steal(),
            TEMPMON: tempmon::TEMPMON::steal(),
            USB_ANALOG: usb_analog::USB_ANALOG::steal(),
            XTALOSC24M: xtalosc24m::XTALOSC24M::steal(),
            USBPHY: usbphy::USBPHY::steal(),
            CSU: csu::CSU::steal(),
            USB: usb::USB::steal(),
            USBNC: usbnc::USBNC::steal(),
            DMA0: dma0::DMA0::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            DCP: dcp::DCP::steal(),
            GPC: gpc::GPC::steal(),
            PGC: pgc::PGC::steal(),
            SRC: src::SRC::steal(),
            CCM: ccm::CCM::steal(),
            ROMC: romc::ROMC::steal(),
            LPUART1: lpuart::LPUART1::steal(),
            LPUART2: lpuart::LPUART2::steal(),
            LPUART3: lpuart::LPUART3::steal(),
            LPUART4: lpuart::LPUART4::steal(),
            LPSPI1: lpspi::LPSPI1::steal(),
            LPSPI2: lpspi::LPSPI2::steal(),
            LPI2C1: lpi2c::LPI2C1::steal(),
            LPI2C2: lpi2c::LPI2C2::steal(),
            FLEXIO1: flexio1::FLEXIO1::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO2: gpio::GPIO2::steal(),
            PWM: pwm::PWM::steal(),
            SPDIF: spdif::SPDIF::steal(),
            SAI1: sai::SAI1::steal(),
            SAI3: sai::SAI3::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            KPP: kpp::KPP::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
