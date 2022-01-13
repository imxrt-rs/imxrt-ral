//! Testing behaviors of take, release, and steal

//
// Test authors: use peripherals that are available
// across all i.MX RT variants. This means no LPUART 5-8,
// no LPSPI 3-4, etc. since they're not available on the
// 1011.
//

#![cfg(all(
    any(
        feature = "imxrt1011",
        feature = "imxrt1015",
        feature = "imxrt1021",
        feature = "imxrt1051",
        feature = "imxrt1052",
        feature = "imxrt1061",
        feature = "imxrt1062",
        feature = "imxrt1064",
    ),
    not(feature = "nosync")
))]

#[test]
fn take_release() {
    let inst = imxrt_ral::lpuart::LPUART1::take().unwrap();
    imxrt_ral::lpuart::LPUART1::release(inst);
}

#[test]
fn double_take() {
    let inst = imxrt_ral::lpuart::LPUART2::take();
    assert!(inst.is_some());
    let empty = imxrt_ral::lpuart::LPUART2::take();
    assert!(empty.is_none());
    imxrt_ral::lpuart::LPUART2::release(inst.unwrap());
}

#[test]
fn steal_take_release() {
    let inst = unsafe { imxrt_ral::lpuart::LPUART3::steal() };
    let empty = imxrt_ral::lpuart::LPUART3::take();
    assert!(empty.is_none());
    imxrt_ral::lpuart::LPUART3::release(inst);

    let inst = imxrt_ral::lpuart::LPUART3::take().unwrap();
    imxrt_ral::lpuart::LPUART3::release(inst);
}

#[test]
#[should_panic(expected = "Released a peripheral which was not taken")]
fn double_release() {
    let taken = imxrt_ral::lpuart::LPUART4::take(); // taken = true
    assert!(taken.is_some());
    let taken = taken.unwrap();

    let stolen = unsafe { imxrt_ral::lpuart::LPUART4::steal() }; // taken = true, again
    imxrt_ral::lpuart::LPUART4::release(stolen); // taken = false

    imxrt_ral::lpuart::LPUART4::release(taken); // panic: not taken
}

// Demonstrates a roundabout way to take two of the same instances.
#[test]
fn two_instances() {
    // Crate A...
    let first_inst = imxrt_ral::lpi2c::LPI2C2::take().unwrap();

    {
        // Pretend that these calls happen in another crate, B...
        let stolen = unsafe { imxrt_ral::lpi2c::LPI2C2::steal() };
        imxrt_ral::lpi2c::LPI2C2::release(stolen);
    }

    // Crate A, later...
    let second_inst = imxrt_ral::lpi2c::LPI2C2::take().unwrap();

    // Might appear to crate A as if two takes() succeeded.
    assert_eq!(&*first_inst as *const _, &*second_inst as *const _);
    imxrt_ral::lpi2c::LPI2C2::release(second_inst);
}
