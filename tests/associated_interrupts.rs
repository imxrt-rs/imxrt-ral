//! Asserts that associated interrupts work

#![cfg(any(
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064",
))]

use imxrt_ral::{dma0, lpuart};
use std::{collections::HashSet, iter::FromIterator};

#[cfg(not(feature = "nosync"))]
#[test]
fn associated_interrupts() {
    let lpuart3 = lpuart::LPUART3::take().unwrap();
    let intrs = lpuart3.interrupts();
    assert_eq!(intrs.len(), 1);
    assert_eq!(intrs[0], imxrt_ral::interrupt::LPUART3);
    lpuart::LPUART3::release(lpuart3);
}

#[test]
fn associated_interrupts_const() {
    const COUNT: usize = lpuart::LPUART3::INTERRUPTS.len();
    const INTERRUPTS: [imxrt_ral::Interrupt; COUNT] = lpuart::LPUART3::INTERRUPTS;

    assert_eq!(COUNT, 1);
    assert_eq!(INTERRUPTS, [imxrt_ral::Interrupt::LPUART3]);
}

#[cfg(feature = "imxrt1011")]
// Sixteen channels, one error.
const EXPECTED_DMA_INTERRUPT_COUNT: usize = 16 + 1;
#[cfg(feature = "imxrt1011")]
const EXPECTED_DMA_INTERRUPTS_UNORDERED: &[imxrt_ral::Interrupt] = &[
    imxrt_ral::Interrupt::DMA_ERROR,
    imxrt_ral::Interrupt::DMA0,
    imxrt_ral::Interrupt::DMA1,
    imxrt_ral::Interrupt::DMA2,
    imxrt_ral::Interrupt::DMA3,
    imxrt_ral::Interrupt::DMA4,
    imxrt_ral::Interrupt::DMA5,
    imxrt_ral::Interrupt::DMA6,
    imxrt_ral::Interrupt::DMA7,
    imxrt_ral::Interrupt::DMA8,
    imxrt_ral::Interrupt::DMA9,
    imxrt_ral::Interrupt::DMA10,
    imxrt_ral::Interrupt::DMA11,
    imxrt_ral::Interrupt::DMA12,
    imxrt_ral::Interrupt::DMA13,
    imxrt_ral::Interrupt::DMA14,
    imxrt_ral::Interrupt::DMA15,
];

#[cfg(not(feature = "imxrt1011"))]
// Thirty two channels, one interrupt signal for two channels, one error.
const EXPECTED_DMA_INTERRUPT_COUNT: usize = 32 / 2 + 1;
#[cfg(not(feature = "imxrt1011"))]
const EXPECTED_DMA_INTERRUPTS_UNORDERED: &[imxrt_ral::Interrupt] = &[
    imxrt_ral::Interrupt::DMA_ERROR,
    imxrt_ral::Interrupt::DMA0_DMA16,
    imxrt_ral::Interrupt::DMA1_DMA17,
    imxrt_ral::Interrupt::DMA2_DMA18,
    imxrt_ral::Interrupt::DMA3_DMA19,
    imxrt_ral::Interrupt::DMA4_DMA20,
    imxrt_ral::Interrupt::DMA5_DMA21,
    imxrt_ral::Interrupt::DMA6_DMA22,
    imxrt_ral::Interrupt::DMA7_DMA23,
    imxrt_ral::Interrupt::DMA8_DMA24,
    imxrt_ral::Interrupt::DMA9_DMA25,
    imxrt_ral::Interrupt::DMA10_DMA26,
    imxrt_ral::Interrupt::DMA11_DMA27,
    imxrt_ral::Interrupt::DMA12_DMA28,
    imxrt_ral::Interrupt::DMA13_DMA29,
    imxrt_ral::Interrupt::DMA14_DMA30,
    imxrt_ral::Interrupt::DMA15_DMA31,
];

#[cfg(not(feature = "nosync"))]
#[test]
fn multiple_associated_interrupts() {
    let dma = dma0::DMA0::take().unwrap();
    let intrs = dma.interrupts();
    assert_eq!(intrs.len(), EXPECTED_DMA_INTERRUPT_COUNT);

    let actual: HashSet<u8> = HashSet::from_iter(intrs.iter().map(|intr| *intr as u8));
    let expected = HashSet::from_iter(
        EXPECTED_DMA_INTERRUPTS_UNORDERED
            .iter()
            .map(|intr| *intr as u8),
    );
    assert_eq!(actual, expected);
    dma0::DMA0::release(dma);
}

#[test]
fn multiple_associated_interrupts_const() {
    const COUNT: usize = dma0::DMA0::INTERRUPTS.len();
    const INTERRUPTS: [imxrt_ral::Interrupt; COUNT] = dma0::DMA0::INTERRUPTS;

    assert_eq!(COUNT, EXPECTED_DMA_INTERRUPT_COUNT);

    let actual: HashSet<u8> = HashSet::from_iter(INTERRUPTS.iter().map(|intr| *intr as u8));
    let expected = HashSet::from_iter(
        EXPECTED_DMA_INTERRUPTS_UNORDERED
            .iter()
            .map(|intr| *intr as u8),
    );
    assert_eq!(actual, expected);
}
