use std::collections::BTreeMap;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::ir::*;
use crate::util::{self, ToSanitizedUpperCase};

pub fn render(_opts: &super::Options, _ir: &IR, d: &Device) -> Result<TokenStream> {
    let num_endings = regex::Regex::new(r"(\d+)$").unwrap();
    let mut out = TokenStream::new();
    let span = Span::call_site();

    let mut interrupts_sorted = d.interrupts.clone();
    interrupts_sorted.sort_by_key(|i| i.value);

    let mut interrupts = TokenStream::new();
    let mut peripherals = TokenStream::new();
    let mut vectors = TokenStream::new();
    let mut names = vec![];

    let mut pos = 0;
    for i in &interrupts_sorted {
        while pos < i.value {
            vectors.extend(quote!(Vector { _reserved: 0 },));
            pos += 1;
        }
        pos += 1;

        let name_uc = Ident::new(&i.name.to_sanitized_upper_case(), span);
        let description = format!(
            "{} - {}",
            i.value,
            i.description
                .as_ref()
                .map(|s| util::respace(s))
                .as_ref()
                .map(|s| util::escape_brackets(s))
                .unwrap_or_else(|| i.name.clone())
        );

        let value = util::unsuffixed(i.value as u64);

        interrupts.extend(quote! {
            #[doc = #description]
            #name_uc = #value,
        });
        vectors.extend(quote!(Vector { _handler: #name_uc },));
        names.push(name_uc);
    }

    let mut block_to_peripherals = BTreeMap::new();
    for peripheral in &d.peripherals {
        let block_name = peripheral
            .block
            .as_ref()
            .expect("All peripherals must have a block");
        let (block_path, _) = super::split_path(block_name);
        let mod_name = block_path
            .last()
            .expect("There's a final component")
            .to_string();
        block_to_peripherals
            .entry(mod_name)
            .or_insert_with(|| (block_path, Vec::new()))
            .1
            .push(peripheral)
    }

    for (mod_name, (block_path, periphs)) in &block_to_peripherals {
        let mut consts = TokenStream::new();
        for peripheral in periphs.iter() {
            if peripheral.base_address == 0 {
                continue;
            }

            let name = Ident::new(&peripheral.name, span);
            let address = util::hex(peripheral.base_address as u64);
            let doc = util::doc(&peripheral.description);

            consts.extend(quote! {
                #doc
                pub const #name: *const RegisterBlock = #address as *const RegisterBlock;
            });
        }

        let import = {
            let block_path = block_path.join("/");
            const BLOCK_MOD: &str = super::BLOCK_MOD;
            let module_path = format!("{BLOCK_MOD}/{block_path}.rs");
            quote! {
                #[path = #module_path]
                mod blocks;
                pub use blocks::*;
            }
        };

        let number_fn: TokenStream;
        let instances = if periphs.len() > 1
            && periphs
                .iter()
                .all(|periph| num_endings.is_match(&periph.name))
        {
            let mut instances = TokenStream::new();
            let mut const_to_num: Vec<TokenStream> = Vec::new();
            for peripheral in periphs.iter() {
                if peripheral.base_address == 0 {
                    continue;
                }
                let name = Ident::new(&peripheral.name, span);
                let num = num_endings.captures(&peripheral.name).unwrap();
                let num = util::unsuffixed(
                    num.get(1)
                        .and_then(|num| str::parse(num.as_str()).ok())
                        .unwrap(),
                );
                const_to_num.push(quote! { (#name, #num), });
                instances.extend(quote! {
                    pub type #name = Instance<#num>;
                    impl crate::private::Sealed for #name {}
                    impl crate::Valid for #name {}

                    impl #name {
                        /// Acquire a vaild, but possibly aliased, instance.
                        ///
                        /// # Safety
                        ///
                        /// See [the struct-level safety documentation](crate::Instance).
                        #[inline]
                        pub const unsafe fn instance() -> Self {
                            Instance::new(#name)
                        }
                    }
                });
            }
            number_fn = quote! {
                /// Returns the instance number `N` for a peripheral instance.
                pub fn number(rb: *const RegisterBlock) -> Option<u8> {
                    [#(#const_to_num)*].into_iter()
                        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
                        .map(|(_, inst)| inst)
                }
            };
            instances
        } else {
            assert!(
                periphs.len() == 1,
                r#"{periphs:#?}
Cannot generate this constified API when there's multiple, un-numbered peripherals.
The implementation doesn't automagically handle this right now. Until this is implemented,
you should use transforms to rename peripherals, putting numbers at the end of the peripheral
name."#
            );
            let peripheral = periphs.first().unwrap();
            if peripheral.base_address != 0 {
                let name = Ident::new(&peripheral.name, span);
                number_fn = quote! {
                    /// Returns the instance number `N` for a peripheral instance.
                    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
                        core::ptr::eq(rb, #name).then_some(0)
                    }
                };
                quote! {
                    pub type #name = Instance<{crate::SOLE_INSTANCE}>;
                    impl crate::private::Sealed for #name {}
                    impl crate::Valid for #name {}
                    impl #name {
                        /// Acquire a vaild, but possibly aliased, instance.
                        ///
                        /// # Safety
                        ///
                        /// See [the struct-level safety documentation](crate::Instance).
                        #[inline]
                        pub const unsafe fn instance() -> Self {
                            Instance::new(#name)
                        }
                    }
                }
            } else {
                number_fn = quote!();
                quote!()
            }
        };

        let mod_name = Ident::new(mod_name, span);
        peripherals.extend(quote! {
            #[path = "."]
            pub mod #mod_name {
                #consts
                #import

                pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
                #instances
                #number_fn
            }
        })
    }

    let n = util::unsuffixed(pos as u64);
    out.extend(quote!(
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Interrupt {
            #interrupts
        }
        pub type interrupt = Interrupt;

        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        #[cfg(all(feature = "rt", target_os = "none"))]
        mod _vectors {
            extern "C" {
                #(fn #names();)*
            }

            pub union Vector {
                _handler: unsafe extern "C" fn(),
                _reserved: u32,
            }

            #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
            #[no_mangle]
            pub static __INTERRUPTS: [Vector; #n] = [
                #vectors
            ];
        }

        #peripherals
    ));

    let cpu = d.cpu.as_ref().expect("There must be a CPU.");
    let bits = util::unsuffixed(u64::from(cpu.nvic_priority_bits));

    out.extend(quote! {
        ///Number available in the NVIC for configuring priority
        pub const NVIC_PRIO_BITS: u8 = #bits;
    });

    //
    // Emit RTIC peripheral struct.
    //
    let mut member_decls = TokenStream::new();
    let mut member_inits = TokenStream::new();
    for (mod_name, (_, peripherals)) in &block_to_peripherals {
        for peripheral in peripherals {
            if peripheral.base_address == 0 {
                continue;
            }

            let name = Ident::new(&peripheral.name, span);
            let mod_name = Ident::new(mod_name, span);
            member_decls.extend(quote! {
                pub #name: #mod_name::#name,
            });
            member_inits.extend(quote! {
                #name: #mod_name::#name::instance(),
            });
        }
    }
    out.extend(quote! {
        /// Instances for all of this device's peripherals.
        ///
        /// Use this if you want a single way to acquire *all* instances
        /// for your device.
        pub struct Instances {
            #member_decls
        }
        impl Instances {
            /// Acquire all peripheral instances.
            ///
            /// # Safety
            ///
            /// Since this calls `instance()` to initialize each of its members,
            /// the `instance()` safety contract applies. See [the `Instance` safety
            /// documentation](crate::Instance) for more information.
            #[inline]
            pub const unsafe fn instances() -> Self {
                Self {
                    #member_inits
                }
            }
        }
    });

    Ok(out)
}
