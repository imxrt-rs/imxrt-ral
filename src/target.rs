//! Routines that are specific to the target device

#[cfg(target_arch = "arm")]
/// Routines on ARM
mod arm {
    pub fn critical_section<R>(f: impl FnOnce() -> R) -> R {
        ::cortex_m::interrupt::free(|_| f())
    }
}

#[cfg(not(target_arch = "arm"))]
/// Routines for your host (supporting testing)
mod host {
    pub fn critical_section<R>(f: impl FnOnce() -> R) -> R {
        use once_cell::sync::Lazy;
        use std::sync::Mutex;
        static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
        let _ = LOCK.lock().unwrap();
        f()
    }
}

#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(not(target_arch = "arm"))]
pub use host::*;
