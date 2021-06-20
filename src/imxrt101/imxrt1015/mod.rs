//! Register access layer for imxrt1015

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
use typenum::*;
pub mod aipstz;
pub mod dcdc;
pub mod pit;
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
pub mod dma0;
pub mod dmamux;
pub mod gpc;
pub use super::instances::pgc;
pub mod ccm;
pub mod src;
pub use super::instances::flexio1;
pub use super::instances::lpuart;
pub use super::instances::romc;
pub mod gpio;
pub mod tmr1;
pub use super::instances::gpt;
pub mod iomuxc;
pub mod ocotp;
pub use super::instances::kpp;
pub mod adc_etc;
pub mod aoi;
pub mod bee;
pub mod dcp;
pub mod enc1;
pub mod flexspi;
pub mod lpi2c;
pub mod lpspi;
pub mod pwm1;
pub mod sai;
pub mod spdif;
pub mod usb;
pub mod usbnc;
pub mod xbara;
pub mod xbarb;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AIPSTZ1: aipstz::Instance<U1>,
    pub AIPSTZ2: aipstz::Instance<U2>,
    pub AIPSTZ3: aipstz::Instance<U3>,
    pub AIPSTZ4: aipstz::Instance<U4>,
    pub DCDC: dcdc::Instance,
    pub PIT: pit::Instance,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::Instance,
    pub IOMUXC_SNVS: iomuxc_snvs::Instance,
    pub IOMUXC_GPR: iomuxc_gpr::Instance,
    pub FLEXRAM: flexram::Instance,
    pub EWM: ewm::Instance,
    pub WDOG1: wdog::Instance<U1>,
    pub WDOG2: wdog::Instance<U2>,
    pub RTWDOG: rtwdog::Instance,
    pub ADC: adc::Instance,
    pub TRNG: trng::Instance,
    pub SNVS: snvs::Instance,
    pub CCM_ANALOG: ccm_analog::Instance,
    pub PMU: pmu::Instance,
    pub TEMPMON: tempmon::Instance,
    pub USB_ANALOG: usb_analog::Instance,
    pub XTALOSC24M: xtalosc24m::Instance,
    pub USBPHY: usbphy::Instance,
    pub CSU: csu::Instance,
    pub DMA0: dma0::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPC: gpc::Instance,
    pub PGC: pgc::Instance,
    pub SRC: src::Instance,
    pub CCM: ccm::Instance,
    pub ROMC: romc::Instance,
    pub LPUART1: lpuart::Instance<U1>,
    pub LPUART2: lpuart::Instance<U2>,
    pub LPUART3: lpuart::Instance<U3>,
    pub LPUART4: lpuart::Instance<U4>,
    pub FLEXIO1: flexio1::Instance,
    pub GPIO1: gpio::Instance<U1>,
    pub GPIO5: gpio::Instance<U5>,
    pub GPIO2: gpio::Instance<U2>,
    pub GPIO3: gpio::Instance<U3>,
    pub TMR1: tmr1::Instance,
    pub GPT1: gpt::Instance<U1>,
    pub GPT2: gpt::Instance<U2>,
    pub OCOTP: ocotp::Instance,
    pub IOMUXC: iomuxc::Instance,
    pub KPP: kpp::Instance,
    pub FLEXSPI: flexspi::Instance,
    pub USB: usb::Instance,
    pub USBNC: usbnc::Instance,
    pub DCP: dcp::Instance,
    pub SPDIF: spdif::Instance,
    pub SAI1: sai::Instance<U1>,
    pub SAI2: sai::Instance<U2>,
    pub SAI3: sai::Instance<U3>,
    pub LPSPI1: lpspi::Instance<U1>,
    pub LPSPI2: lpspi::Instance<U2>,
    pub ADC_ETC: adc_etc::Instance,
    pub AOI: aoi::Instance,
    pub XBARA: xbara::Instance,
    pub XBARB: xbarb::Instance,
    pub ENC1: enc1::Instance,
    pub PWM1: pwm1::Instance,
    pub BEE: bee::Instance,
    pub LPI2C1: lpi2c::Instance<U1>,
    pub LPI2C2: lpi2c::Instance<U2>,
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
            FLEXIO1: flexio1::FLEXIO1::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO2: gpio::GPIO2::steal(),
            GPIO3: gpio::GPIO3::steal(),
            TMR1: tmr1::TMR1::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            KPP: kpp::KPP::steal(),
            FLEXSPI: flexspi::FLEXSPI::steal(),
            USB: usb::USB::steal(),
            USBNC: usbnc::USBNC::steal(),
            DCP: dcp::DCP::steal(),
            SPDIF: spdif::SPDIF::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SAI3: sai::SAI3::steal(),
            LPSPI1: lpspi::LPSPI1::steal(),
            LPSPI2: lpspi::LPSPI2::steal(),
            ADC_ETC: adc_etc::ADC_ETC::steal(),
            AOI: aoi::AOI::steal(),
            XBARA: xbara::XBARA::steal(),
            XBARB: xbarb::XBARB::steal(),
            ENC1: enc1::ENC1::steal(),
            PWM1: pwm1::PWM1::steal(),
            BEE: bee::BEE::steal(),
            LPI2C1: lpi2c::LPI2C1::steal(),
            LPI2C2: lpi2c::LPI2C2::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
