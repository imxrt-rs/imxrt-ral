mod block;
mod device;
mod fieldset;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::ir::*;

struct Module {
    items: TokenStream,
    children: BTreeMap<String, Module>,
    public: bool,
    fs_only: bool,
    reexport: bool,
    conditional_feature: Option<String>,
}

impl Module {
    fn new() -> Self {
        Self {
            // Default mod contents
            items: quote!(),
            children: BTreeMap::new(),
            public: true,
            fs_only: false,
            reexport: false,
            conditional_feature: None,
        }
    }

    fn mark_private(&mut self) -> &mut Module {
        self.public = false;
        self
    }

    fn mark_fs_only(&mut self) -> &mut Module {
        self.fs_only = true;
        self
    }

    fn mark_reexport(&mut self) -> &mut Module {
        self.reexport = true;
        self
    }

    fn conditional_on(&mut self, feature: &str) -> &mut Module {
        self.conditional_feature = Some(feature.into());
        self
    }

    fn get_by_path(&mut self, path: &[&str]) -> &mut Module {
        if path.is_empty() {
            return self;
        }

        self.children
            .entry(path[0].to_owned())
            .or_insert_with(Module::new)
            .get_by_path(&path[1..])
    }

    fn render(self, path: &Path) -> Result<()> {
        let span = Span::call_site();

        let mut res = TokenStream::new();
        res.extend(self.items);

        for (name, module) in self.children.into_iter() {
            let name = Ident::new(&name, span);

            let subpath = if let Some(parent) = path.parent() {
                if path.file_name() == Some(std::ffi::OsStr::new("lib.rs")) {
                    parent.join(format! {"{name}.rs"})
                } else {
                    parent
                        .join(path.file_stem().unwrap())
                        .join(format!("{name}.rs"))
                }
            } else {
                PathBuf::from(format!("{name}.rs"))
            };

            if !module.fs_only {
                let privacy = if module.public { quote!(pub) } else { quote!() };
                let conditional = if let Some(feature) = &module.conditional_feature {
                    quote!(#[cfg(feature = #feature)])
                } else {
                    quote!()
                };
                let reexport = if module.reexport {
                    quote!(pub use #name::*;)
                } else {
                    quote!()
                };
                module.render(&subpath)?;
                let file_path = format!("{name}.rs");
                res.extend(quote! {
                    #conditional
                    #[path = #file_path]
                    #privacy mod #name;
                    #conditional
                    #reexport
                });
            } else {
                module.render(&subpath)?;
            }
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if !self.fs_only {
            fs::write(path, res.to_string().as_bytes())?;
        }
        Ok(())
    }
}

pub enum CommonModule {
    Builtin,
    External(TokenStream),
}

pub struct Options {
    pub module_root: PathBuf,
    pub weak_syms: bool,
}

const BLOCK_MOD: &str = "blocks";

pub fn render(ir: &IR, opts: &Options) -> Result<()> {
    let mut root = Module::new();
    root.items = TokenStream::new(); // Remove default contents
    root.get_by_path(&[BLOCK_MOD]).mark_fs_only();

    root.items.extend(quote!(
        #![doc = include_str!("../doc.md")]
        #![no_std]
        #![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, clippy::self_named_constructors, clippy::module_inception)]

        pub use ral_registers::{RWRegister, RORegister, WORegister, read_reg, write_reg, modify_reg};

        /// An owned peripheral of type `T`, instance `N`.
        ///
        /// Fabricating an `Instance` is always `unsafe`. An owner of an
        /// `Instance` may assume that
        ///
        /// - the underlying pointer points to a static register block of type `T`.
        /// - the instance number `N` properly describes the peripheral instance.
        /// - they own _all_ registers pointed at by `T`.
        ///
        /// Owners use this guarantee to safely access the peripheral registers.
        /// However, nothing guarantees any of these except for your diligence.
        ///
        /// Constructing an `Instance` is zero cost. Additionally, `Instance` is transparent
        /// and amenable to null-pointer optimizations.
        ///
        /// See the package-level documentation for more information on fabricating
        /// instances.
        ///
        /// # Safety of `new()`.
        ///
        /// By calling `new()`, you claim
        ///
        /// 1. `ptr` points to static memory that can be described by a type `T`.
        /// 2. The instance number `N` correctly describes `ptr`.
        /// 3. You are becoming the sole owner of this instance.
        ///
        /// # Safety of `instance()`
        ///
        /// The various `instance()` methods handle safety concerns 1 and 2 from `new()`.
        /// By their construction, each `instance()` implementation provides a pointer to valid
        /// peripheral memory, and associates the correct `N` with that pointer. Therefore,
        /// you're only responsible for ensuring safety concern 3 from `new()`.
        #[repr(transparent)]
        pub struct Instance<T, const N: u8> {
            ptr: core::ptr::NonNull<T>,
        }

        impl<T, const N: u8> core::ops::Deref for Instance<T, N> {
            type Target = T;
            #[inline]
            fn deref(&self) -> &Self::Target {
                // Safety: User provided a pointer that points to static MMIO.
                // This implies non-null, initialized, aligned, and dereferenceable.
                unsafe { self.ptr.as_ref() }
            }
        }

        impl<T, const N: u8> Instance<T, N> {
            /// Create an arbitrary `Instance` from a pointer to `T`.
            ///
            /// # Safety
            ///
            /// See [the struct docs](Instance) for the safety contract.
            #[inline]
            pub const unsafe fn new(ptr: *const T) -> Self {
                // Casting *const _ to *mut _ is OK. The mutable pointer never
                // escapes Instance.
                Self { ptr: core::ptr::NonNull::new_unchecked(ptr as *mut _) }
            }
        }

        unsafe impl<T, const N: u8> Send for Instance<T, N> {}

        /// The instance number for a peripheral singleton.
        ///
        /// If your peripheral only has one instance, it's given
        /// this number. The CCM peripheral is a good example of
        /// a peripheral that uses this constant.
        ///
        /// See the package documentation for more information on
        /// this constant.
        pub const SOLE_INSTANCE: u8 = 0u8;
        mod private {
            pub trait Sealed {}
        }

        /// Vouches for an `Instance<T, N>`'s validity.
        ///
        /// This trait is implemented for all `Instance<T, N>` supported
        /// by your chip. Note that the implementation may change when
        /// selecting new chip features. For instance, i.MX RT 1011 chips
        /// do not have LPUART 4 through 8. So, `Valid` is _not_ implemented
        /// for `lpuart::Instance<4>` through `lpuart::Instance<8>`.
        ///
        /// See the package documentation for more information on how
        /// to use this trait in your APIs.
        pub trait Valid : private::Sealed {}
    ));

    let mut root_blocks = HashSet::new();
    for (p, d) in ir.devices.iter() {
        root_blocks.extend(
            d.peripherals
                .iter()
                .filter_map(|peripheral| peripheral.block.as_ref()),
        );
        let mods = p.split("::").collect::<Vec<_>>();
        root.get_by_path(&mods)
            .items
            .extend(device::render(opts, ir, d)?);
    }

    for root_block in root_blocks {
        let b = ir.blocks.get(root_block).unwrap();
        let (mods, _) = split_path(root_block);
        root.get_by_path(&[BLOCK_MOD])
            .get_by_path(&mods)
            .items
            .extend(block::render(ir, b, root_block)?);
    }

    for (dev_mod_name, dev_mod) in root.children.iter_mut().filter(|(k, _)| *k != BLOCK_MOD) {
        dev_mod
            .mark_private()
            .conditional_on(dev_mod_name)
            .mark_reexport();
    }
    for block_dev_mod in root.get_by_path(&[BLOCK_MOD]).children.values_mut() {
        block_dev_mod.mark_fs_only();
    }

    root.render(&opts.module_root)?;
    weak_syms(opts, ir)?;
    Ok(())
}

fn split_path(s: &str) -> (Vec<&str>, &str) {
    let mut v: Vec<&str> = s.split("::").collect();
    let n = v.pop().unwrap();
    (v, n)
}

/// Generate a linker script of weak symbols for interrupt handlers.
fn weak_syms(opts: &Options, ir: &IR) -> Result<()> {
    if !opts.weak_syms {
        return Ok(());
    }

    for (name, device) in &ir.devices {
        if name.is_empty() {
            continue;
        }

        let mut interrupts = device.interrupts.clone();
        interrupts.sort_by_key(|intr| intr.value);

        let mut path = opts.module_root.parent().unwrap().join(name);
        path.set_extension("x");

        let file = fs::File::create(path)?;
        let mut file = io::BufWriter::new(file);

        for intr in interrupts {
            writeln!(file, "PROVIDE({} = DefaultHandler);", intr.name)?;
        }
    }

    Ok(())
}
