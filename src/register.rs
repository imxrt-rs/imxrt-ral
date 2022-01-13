pub use ral_registers::{
    modify_reg, read_reg, write_reg, RORegister, RWRegister, UnsafeRORegister, UnsafeRWRegister,
    UnsafeWORegister, WORegister,
};

/// Reset a RWRegister, UnsafeRWRegister, WORegister, or UnsafeWORegister to its reset value.
///
/// # Examples
/// ```rust,no_run
/// # use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg}; fn main() {
/// // Safely acquire the peripheral instance (will panic if already acquired)
/// let gpio = imxrt_ral::gpio::GPIO1::take().unwrap();
///
/// // Reset ICR9 and ICR10 to their reset state
/// reset_reg!(imxrt_ral::gpio, gpio, GPIO1, ICR1, ICR9, ICR10);
///
/// // Reset the entire GPIO1.ICR1 to its reset state
/// reset_reg!(imxrt_ral::gpio, gpio, GPIO1, ICR1);
/// # }
/// ```
///
/// # Usage
/// Like `write_reg!`, this macro can be used in two ways, either resetting the entire register
/// or just resetting specific fields within in. The register or fields are written with their
/// reset values.
///
/// In both cases, the first arguments are:
/// * the path to the peripheral module: `imxrt_ral::gpio`,
/// * a reference to the instance of that peripheral: 'gpio' (anything which dereferences to
///   `RegisterBlock`, such as `Instance`, `&Instance`, `&RegisterBlock`, or
///   `*const RegisterBlock`),
/// * the module for the instance of that peripheral: `GPIO1`,
/// * the register you wish to access: `ICR1` (a field on the `RegisterBlock`).
///
/// In the whole-register usage, that's it:
/// ```rust,no_run
/// # use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg}; fn main() {
/// # let gpio = imxrt_ral::gpio::GPIO1::take().unwrap();
/// // Reset the entire GPIO1.ICR1
/// reset_reg!(imxrt_ral::gpio, gpio, GPIO1, ICR1);
/// # }
/// ```
///
/// Otherwise, the remaining arguments are each field names:
/// ```rust,no_run
/// # use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg}; fn main() {
/// # let gpio = imxrt_ral::gpio::GPIO1::take().unwrap();
/// // Reset fields in GPIO1 and GPIO2
/// reset_reg!(imxrt_ral::gpio, gpio, GPIO1, ICR1, ICR5, ICR6, ICR7);
/// reset_reg!(imxrt_ral::gpio, gpio, GPIO2, ICR1, ICR1, ICR2);
/// # }
/// ```
///
/// The second form is only available to RWRegister and UnsafeRWRegister, since `.read()` is
/// not available for WORegister and UnsafeWORegister.
///
/// This macro expands to calling `(*$instance).$register.write(value)`, where
/// `value` is either the register's reset value, or the current read value of the register
/// masked appropriately and combined with the reset value for each field.
///
/// # Safety
/// This macro will require an unsafe function or block when used with an UnsafeRWRegister or
/// UnsafeRORegister, but not if used with RWRegister or RORegister.
///
/// When run in an unsafe context, peripheral instances are directly accessible without requiring
/// having called `take()` beforehand:
/// ```rust,no_run
/// # use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg}; fn main() {
/// unsafe { reset_reg!(imxrt_ral::gpio, GPIO1, GPIO1, ICR1) };
/// # }
/// ```
/// This works because `GPIO1` is a `*const RegisterBlock` in the `imxrt_ral::gpio` module;
/// and the macro brings such constants into scope and then dereferences the provided reference.
///
/// Note that the second argument is a `*const` and the third is a path; despite both being written
/// `GPIO1` they are not the same thing.
#[macro_export]
macro_rules! reset_reg {
    ( $periph:path, $instance:expr, $instancety:ty, $reg:ident, $( $field:ident ),+ ) => {{
        #[allow(unused_imports)]
        use $periph::{*};
        #[allow(unused_imports)]
        (*$instance).$reg.write({
            let resetmask: u32 = $({ use $periph::{$reg::$field::mask}; mask }) | *;
            ((*$instance).$reg.read() & !resetmask) | (<$instancety>::reset.$reg & resetmask)
        });
    }};
    ( $periph:path, $instance:expr, $instancety:ty, $reg:ident ) => {{
        #[allow(unused_imports)]
        use $periph::{*};
        (*$instance).$reg.write(<$instancety>::reset.$reg);
    }};
}
