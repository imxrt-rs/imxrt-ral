use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util;

pub fn render(ir: &IR, fs: &FieldSet) -> Result<TokenStream> {
    let span = Span::call_site();
    let mut items = TokenStream::new();

    let ty = match fs.bit_size {
        BitSize(1..=8) => quote!(u8),
        BitSize(9..=16) => quote!(u16),
        BitSize(17..=32) => quote!(u32),
        BitSize(33..=64) => quote!(u64),
        BitSize(invalid) => anyhow::bail!("Invalid bit_size {invalid}"),
    };

    for f in &fs.fields {
        anyhow::ensure!(
            f.array.is_none(),
            "Field {} is an array, and that's not supported",
            f.name
        );

        let name = Ident::new(&f.name, span);
        let bit_offset = proc_macro2::Literal::u32_unsuffixed(f.bit_offset);
        let mask = util::hex(1u64.wrapping_shl(f.bit_size.0).wrapping_sub(1));
        let doc = util::doc(&f.description);

        let enum_tokenize = |enm: &Option<String>| -> TokenStream {
            enm.as_ref()
                .and_then(|path| ir.enums.get(path))
                .map(|enm| {
                    let mut items = TokenStream::new();
                    for e in &enm.variants {
                        let name = Ident::new(&e.name, span);
                        let value = util::hex(e.value);
                        let doc = util::doc(&e.description);
                        items.extend(quote!(
                            #doc
                            pub const #name: #ty = #value;
                        ));
                    }
                    items
                })
                .unwrap_or_else(TokenStream::new)
        };

        let reads = enum_tokenize(&f.enum_read);
        let writes = enum_tokenize(&f.enum_write);
        let reads_writes = enum_tokenize(&f.enum_readwrite);

        items.extend(quote! {
            #doc
            pub mod #name {
                pub const offset: #ty = #bit_offset;
                pub const mask: #ty = #mask << offset;
                pub mod R { #reads }
                pub mod W { #writes }
                pub mod RW { #reads_writes }
            }
        });
    }

    Ok(quote! { #items })
}
