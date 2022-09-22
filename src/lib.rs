# ! [doc = include_str ! ("../doc.md")]
#![no_std]
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::self_named_constructors,
    clippy::module_inception
)]
pub use ral_registers::{modify_reg, read_reg, write_reg, RORegister, RWRegister, WORegister};
#[doc = r" An owned peripheral of type `T`, instance `N`."]
#[doc = r""]
#[doc = r" Fabricating an `Instance` is always `unsafe`. An owner of an"]
#[doc = r" `Instance` may assume that"]
#[doc = r""]
#[doc = r" - the underlying pointer points to a static register block of type `T`."]
#[doc = r" - the instance number `N` properly describes the peripheral instance."]
#[doc = r" - they own _all_ registers pointed at by `T`."]
#[doc = r""]
#[doc = r" Owners use this guarantee to safely access the peripheral registers."]
#[doc = r" However, nothing guarantees any of these except for your diligence."]
#[doc = r""]
#[doc = r" Constructing an `Instance` is zero cost. Additionally, `Instance` is transparent"]
#[doc = r" and amenable to null-pointer optimizations."]
#[doc = r""]
#[doc = r" See the package-level documentation for more information on fabricating"]
#[doc = r" instances."]
#[doc = r""]
#[doc = r" # Safety of `new()`."]
#[doc = r""]
#[doc = r" By calling `new()`, you claim"]
#[doc = r""]
#[doc = r" 1. `ptr` points to static memory that can be described by a type `T`."]
#[doc = r" 2. The instance number `N` correctly describes `ptr`."]
#[doc = r" 3. You are becoming the sole owner of this instance."]
#[doc = r""]
#[doc = r" # Safety of `instance()`"]
#[doc = r""]
#[doc = r" The various `instance()` methods handle safety concerns 1 and 2 from `new()`."]
#[doc = r" By their construction, each `instance()` implementation provides a pointer to valid"]
#[doc = r" peripheral memory, and associates the correct `N` with that pointer. Therefore,"]
#[doc = r" you're only responsible for ensuring safety concern 3 from `new()`."]
#[repr(transparent)]
pub struct Instance<T, const N: u8> {
    ptr: core::ptr::NonNull<T>,
}
impl<T, const N: u8> core::ops::Deref for Instance<T, N> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T, const N: u8> Instance<T, N> {
    #[doc = r" Create an arbitrary `Instance` from a pointer to `T`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" See [the struct docs](Instance) for the safety contract."]
    #[inline]
    pub const unsafe fn new(ptr: *const T) -> Self {
        Self {
            ptr: core::ptr::NonNull::new_unchecked(ptr as *mut _),
        }
    }
}
unsafe impl<T, const N: u8> Send for Instance<T, N> {}
#[doc = r" The instance number for a peripheral singleton."]
#[doc = r""]
#[doc = r" If your peripheral only has one instance, it's given"]
#[doc = r" this number. The CCM peripheral is a good example of"]
#[doc = r" a peripheral that uses this constant."]
#[doc = r""]
#[doc = r" See the package documentation for more information on"]
#[doc = r" this constant."]
pub const SOLE_INSTANCE: u8 = 0u8;
mod private {
    pub trait Sealed {}
}
#[doc = r" Vouches for an `Instance<T, N>`'s validity."]
#[doc = r""]
#[doc = r" This trait is implemented for all `Instance<T, N>` supported"]
#[doc = r" by your chip. Note that the implementation may change when"]
#[doc = r" selecting new chip features. For instance, i.MX RT 1011 chips"]
#[doc = r" do not have LPUART 4 through 8. So, `Valid` is _not_ implemented"]
#[doc = r" for `lpuart::Instance<4>` through `lpuart::Instance<8>`."]
#[doc = r""]
#[doc = r" See the package documentation for more information on how"]
#[doc = r" to use this trait in your APIs."]
pub trait Valid: private::Sealed {}
#[cfg(feature = "imxrt1011")]
#[path = "imxrt1011.rs"]
mod imxrt1011;
#[cfg(feature = "imxrt1011")]
pub use imxrt1011::*;
#[cfg(feature = "imxrt1015")]
#[path = "imxrt1015.rs"]
mod imxrt1015;
#[cfg(feature = "imxrt1015")]
pub use imxrt1015::*;
#[cfg(feature = "imxrt1021")]
#[path = "imxrt1021.rs"]
mod imxrt1021;
#[cfg(feature = "imxrt1021")]
pub use imxrt1021::*;
#[cfg(feature = "imxrt1051")]
#[path = "imxrt1051.rs"]
mod imxrt1051;
#[cfg(feature = "imxrt1051")]
pub use imxrt1051::*;
#[cfg(feature = "imxrt1052")]
#[path = "imxrt1052.rs"]
mod imxrt1052;
#[cfg(feature = "imxrt1052")]
pub use imxrt1052::*;
#[cfg(feature = "imxrt1061")]
#[path = "imxrt1061.rs"]
mod imxrt1061;
#[cfg(feature = "imxrt1061")]
pub use imxrt1061::*;
#[cfg(feature = "imxrt1062")]
#[path = "imxrt1062.rs"]
mod imxrt1062;
#[cfg(feature = "imxrt1062")]
pub use imxrt1062::*;
#[cfg(feature = "imxrt1064")]
#[path = "imxrt1064.rs"]
mod imxrt1064;
#[cfg(feature = "imxrt1064")]
pub use imxrt1064::*;
#[cfg(feature = "imxrt1176_cm4")]
#[path = "imxrt1176_cm4.rs"]
mod imxrt1176_cm4;
#[cfg(feature = "imxrt1176_cm4")]
pub use imxrt1176_cm4::*;
#[cfg(feature = "imxrt1176_cm7")]
#[path = "imxrt1176_cm7.rs"]
mod imxrt1176_cm7;
#[cfg(feature = "imxrt1176_cm7")]
pub use imxrt1176_cm7::*;
