//! Generates a register block, along with submodules for register fields.
//!
//! Recursively expands dependent blocks that are part of the module. This
//! means that the input to [`render`] is expected to be a root block, or
//! a block that is not a sub-block of another block.

use std::num::NonZeroUsize;

use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir;
use crate::util;

/// A primitive size for a (reserved) register.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
enum Size {
    U8 = 8,
    U16 = 16,
    U32 = 32,
    U64 = 64,
}

impl Size {
    const fn bits(self) -> usize {
        self as usize
    }
    const fn bytes(self) -> usize {
        self.bits() / 8
    }
    fn type_token(self) -> TokenStream {
        match self {
            Self::U8 => quote!(u8),
            Self::U16 => quote!(u16),
            Self::U32 => quote!(u32),
            Self::U64 => quote!(u64),
        }
    }
    fn from_bit_size(bit_size: ir::BitSize) -> Self {
        match bit_size {
            ir::BitSize(8) => Self::U8,
            ir::BitSize(16) => Self::U16,
            ir::BitSize(32) => Self::U32,
            ir::BitSize(64) => Self::U64,
            ir::BitSize(invalid) => panic!("Invalid register bit size {invalid}"),
        }
    }
}

/// A register block.
///
/// Any stride necessary to meet the requirements of a cluster array
/// are implicitly expressed with a final reservation member at the back
/// of the members collection. Meaning: the stride and size of the block
/// are equal.
///
/// A single `Block` allocation holds the layout for all of its dependent
/// sub-blocks. To find sub-block layouts, scan the `members` for a block,
/// and recurse.
#[derive(Debug)]
struct Block<'a> {
    /// Module name.
    module: String,
    /// Type documentation
    doc: Option<&'a str>,
    /// Members.
    ///
    /// If a block requires sub-blocks, they're contained within
    /// this collection.
    members: Members<'a>,
}

/// Produces a collection of block members with reservations.
///
/// This is where struct layout happens.
fn layout_members<'ir>(
    items: &'ir [ir::BlockItem],
    ir: &'ir ir::IR,
    reservation_id: &mut usize,
) -> Members<'ir> {
    let mut registers: Vec<_> = items
        .iter()
        .flat_map(|item| Member::expand(item, ir))
        .collect();

    // Order by their location in the block.
    //
    // If registers are at the same location, prefer the alias that
    // is read-write.
    registers.sort_by(|left, right| {
        let offsets = left.offset().cmp(&right.offset());
        match (left, right) {
            (Member::Register(left), Member::Register(right)) => {
                offsets.then(left.access.cmp(&right.access))
            }
            _ => offsets,
        }
    });

    // Drop aliasing registers.
    registers.dedup_by(|left, right| left.offset() == right.offset());

    // Insert reservations.
    let mut members: Vec<Member> = Vec::new();
    for register in registers {
        let offset = members
            .last()
            .map(|mem| mem.offset() + mem.size_bytes())
            .unwrap_or(0usize);
        if offset != register.offset() {
            assert!(register.offset() > offset);
            members.push(Member::Reserved {
                len: register.offset() - offset,
                offset,
                id: *reservation_id,
            });
            *reservation_id += 1;
        }
        members.push(register)
    }

    members
}

/// Sanity check of the block layout.
///
/// Panics if there's an issue.
#[cfg(debug_assertions)]
fn check_layout(members: &[Member], path: &str) {
    // Expand registers, modeling each as a range.
    type RegRange = std::ops::Range<usize>;

    fn recurse(members: &[Member], registers: &mut Vec<RegRange>, global_offset: usize) {
        for member in members {
            match member {
                Member::Register(reg) => {
                    for idx in 0..reg.len {
                        registers.push(RegRange {
                            start: global_offset + reg.offset + idx * reg.size.bytes(),
                            end: global_offset + reg.offset + (idx + 1) * reg.size.bytes(),
                        })
                    }
                }
                Member::Reserved { len, offset, .. } => registers.push(RegRange {
                    start: global_offset + *offset,
                    end: global_offset + *offset + *len,
                }),
                Member::Block {
                    block, len, offset, ..
                } => {
                    for idx in 0..*len {
                        recurse(
                            &block.members,
                            registers,
                            global_offset + *offset + idx * block.size_bytes(),
                        );
                    }
                }
            }
        }
    }

    let mut registers: Vec<RegRange> = Vec::new();
    recurse(members, &mut registers, 0);

    for (idx, reg) in registers.iter().enumerate() {
        for (jdx, seg) in registers.iter().enumerate() {
            if idx != jdx {
                for r in reg.clone() {
                    if seg.contains(&r) {
                        panic!(
                            r#"{members:#?}
There's an issue in the '{path}' block layout(s).
This routine flattens registers from blocks, ensuring
that there's no register overlap. If you're reading this
panic message, it's because there's likely overlap.
The questionable block is printed above this message.
Evaluate its layout, and compare it with the SVD.
"#
                        );
                    }
                }
            }
        }
    }
}

impl<'ir> Block<'ir> {
    /// Allocate a new block.
    ///
    /// `path` is the IR path, and `block` is the associated block. A stride
    /// of `None` prevents the routine from inserting any padding at the back
    /// of the block to meet a cluster stride.
    fn new(
        path: &'ir str,
        block: &'ir ir::Block,
        ir: &'ir ir::IR,
        stride: Option<NonZeroUsize>,
    ) -> Self {
        let module = path.split("::").last().unwrap().to_lowercase();
        let mut reservation_id = 0usize;
        let members = layout_members(&block.items, ir, &mut reservation_id);

        let mut block = Self {
            module,
            doc: block.description.as_ref().map(String::as_ref),
            members,
        };

        // Jam some padding in the back to meet the stride.
        if let Some(stride) = stride {
            let size = block.size_bytes();
            assert!(
                stride.get() >= size,
                "Expecting that we need to insert padding or do nothing, but it seems we need to take it away...?"
            );

            let padding_bytes = stride.get() - size;
            if padding_bytes > 0 {
                block.members.push(Member::Reserved {
                    id: reservation_id,
                    len: padding_bytes,
                    offset: block
                        .members
                        .last()
                        .map(|mem| mem.offset() + mem.size_bytes())
                        .unwrap_or(0usize),
                });
            }
        }

        #[cfg(debug_assertions)]
        check_layout(&block.members, path);

        block
    }
}

impl Block<'_> {
    fn size_bytes(&self) -> usize {
        self.members.iter().map(|mem| mem.size_bytes()).sum()
    }
    fn render_into(&self, tokens: &mut TokenStream) {
        let members = self
            .members
            .iter()
            .fold(TokenStream::new(), |mut tokens, member| {
                member.render_into(&mut tokens);
                tokens
            });
        let doc = util::doc(&self.doc.map(ToString::to_string));
        tokens.extend(quote! {
            #doc
            #[repr(C)]
            pub struct RegisterBlock {
                #members
            }
        });
    }
    fn subblocks(&self) -> impl Iterator<Item = &Block> {
        self.members.iter().filter_map(|mem| match mem {
            Member::Block { block, .. } => Some(block),
            _ => None,
        })
    }
    fn registers(&self) -> impl Iterator<Item = &Register> {
        self.members.iter().filter_map(|mem| match mem {
            Member::Register(reg) => Some(reg),
            _ => None,
        })
    }
}

/// A register (array).
#[derive(Debug)]
struct Register<'a> {
    /// Register name.
    ///
    /// Expands to the struct member name.
    name: String,
    /// Size of the register.
    size: Size,
    /// How may registers?
    ///
    /// If `len` is one, the implementation emits a scalar (non-array)
    /// type.
    len: usize,
    /// Optional documentation.
    doc: Option<&'a str>,
    /// Access.
    access: ir::Access,
    /// Key to the associated fieldset.
    fieldset: Option<&'a str>,
    /// Offset of this register within the block.
    offset: usize,
}

/// A struct member.
#[derive(Debug)]
enum Member<'a> {
    /// A useful register.
    Register(Register<'a>),
    /// A reserved register.
    ///
    /// Always a byte array with some `len` of bytes.
    Reserved {
        /// Arbitrary ID for the reserved register.
        ///
        /// Assigned when specifying the block. Only
        /// used to generate a unique identifier for the
        /// member name.
        id: usize,
        /// How many bytes to reserve.
        len: usize,
        /// Byte position in the block.
        offset: usize,
    },
    /// A cluster, or another register subblock.
    Block {
        /// Register layout for the block.
        block: Block<'a>,
        /// How many subblocks? Always greater than zero.
        ///
        /// If one, the implementation emits a single struct
        /// instead of an array.
        len: usize,
        /// The member name.
        ///
        /// Differs from the module name; this expands to
        /// the struct member identifier.
        name: String,
        /// The member documentation.
        ///
        /// Differs from the type documentation.
        doc: Option<&'a str>,
        /// Offset of this block within the parent block.
        offset: usize,
    },
}
type Members<'a> = Vec<Member<'a>>;

impl<'ir> Member<'ir> {
    /// Expands a block into one or more members.
    ///
    /// The returned collection is never empty.
    fn expand(block_item: &'ir ir::BlockItem, ir: &'ir ir::IR) -> Vec<Self> {
        let name = block_item.name.as_str();
        let offset = block_item.byte_offset as usize;
        let doc = block_item.description.as_deref();

        match (&block_item.array, &block_item.inner) {
            // Individual register.
            (None, ir::BlockItemInner::Register(reg)) => {
                vec![Self::Register(Register {
                    name: name.into(),
                    size: Size::from_bit_size(reg.bit_size),
                    len: 1,
                    doc,
                    access: reg.access.clone(),
                    fieldset: reg.fieldset.as_deref(),
                    offset,
                })]
            }
            // Array of registers with contiguous allocation.
            (
                Some(ir::Array::Regular(ir::RegularArray { len, stride })),
                ir::BlockItemInner::Register(reg),
            ) if ir::BitSize(*stride * 8) == reg.bit_size => {
                vec![Self::Register(Register {
                    name: name.into(),
                    size: Size::from_bit_size(reg.bit_size),
                    len: *len as usize,
                    doc,
                    access: reg.access.clone(),
                    fieldset: reg.fieldset.as_deref(),
                    offset,
                })]
            }
            // "Array" of registers, but they're not contiguous. Describe them as
            // individual registers.
            (
                Some(ir::Array::Regular(ir::RegularArray { len, stride })),
                ir::BlockItemInner::Register(reg),
            ) => (0..*len as usize)
                .map(|idx| {
                    Self::Register(Register {
                        name: format!("{name}{idx}"),
                        size: Size::from_bit_size(reg.bit_size),
                        len: 1,
                        doc,
                        access: reg.access.clone(),
                        fieldset: reg.fieldset.as_deref(),
                        offset: offset + idx * *stride as usize,
                    })
                })
                .collect(),
            // A cluster.
            (
                Some(ir::Array::Regular(ir::RegularArray { len, stride })),
                ir::BlockItemInner::Block(ir::BlockItemBlock { block }),
            ) => vec![Self::Block {
                block: Block::new(
                    block,
                    ir.blocks.get(block).unwrap(),
                    ir,
                    NonZeroUsize::new(*stride as usize),
                ),
                len: *len as usize,
                name: name.to_lowercase(),
                doc,
                offset,
            }],
            (Some(ir::Array::Cursed(_)), _) => {
                panic!("Not yet handling a cursed array. I'd rather not spread the curse.");
            }
            (None, ir::BlockItemInner::Block(_)) => {
                panic!("Unexpected cluster without a stride");
            }
        }
    }
}

impl Member<'_> {
    /// Returns the size of the member allocation.
    fn size_bytes(&self) -> usize {
        match self {
            Self::Register(Register { size, len, .. }) => size.bytes() * len,
            Self::Reserved { len, .. } => Size::U8.bytes() * len,
            Self::Block { block, len, .. } => block.size_bytes() * *len,
        }
    }

    fn offset(&self) -> usize {
        match self {
            Self::Register(Register { offset, .. }) => *offset,
            Self::Block { offset, .. } => *offset,
            Self::Reserved { offset, .. } => *offset,
        }
    }

    /// Render this member into a token stream.
    ///
    /// This does not render a sub-block; it only inserts a member name and type
    /// for the parent block (self).
    fn render_into(&self, tokens: &mut TokenStream) {
        match self {
            Self::Reserved { id, len, .. } => {
                assert!(*len > 0, "There's at least one reservation");
                let ty = Size::U8.type_token();
                let reservation = quote::format_ident!("_reserved{}", *id);
                let len = util::hex(*len as u64);
                tokens.extend(quote! {
                    #reservation: [#ty; #len],
                });
            }
            Self::Register(Register {
                name,
                size,
                len,
                doc,
                access,
                ..
            }) => {
                assert!(*len > 0, "There's at least one register");
                let register = match access {
                    ir::Access::Read => quote!(crate::RORegister),
                    ir::Access::Write => quote!(crate::WORegister),
                    ir::Access::ReadWrite => quote!(crate::RWRegister),
                };
                let reg_ty = size.type_token();
                let ty = if *len == 1 {
                    quote!(#register<#reg_ty>)
                } else {
                    quote!([#register<#reg_ty>; #len])
                };
                let span = Span::call_site();
                let name = Ident::new(name, span);
                let doc = util::doc(&doc.map(ToString::to_string));
                tokens.extend(quote! {
                    #doc
                    pub #name: #ty,
                })
            }
            Self::Block {
                len,
                name,
                doc,
                block,
                ..
            } => {
                assert!(*len > 0, "There's at least one block");
                let span = Span::call_site();
                let scalar = Ident::new(&block.module, span);
                let ty = if *len == 1 {
                    quote!(#scalar::RegisterBlock)
                } else {
                    quote!([#scalar::RegisterBlock; #len])
                };
                let doc = util::doc(&doc.map(ToString::to_string));
                let field = Ident::new(&name.to_uppercase(), span);
                tokens.extend(quote! {
                    #doc
                    pub #field: #ty,
                })
            }
        }
    }
}

/// Renders a block module, recursing to render all sub-block modules.
fn render_module(block: &Block, ir: &ir::IR) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    block.render_into(&mut tokens);
    block.registers().try_for_each(|reg| -> Result<()> {
        if let Some(fieldset) = reg
            .fieldset
            .as_ref()
            .and_then(|fieldset| ir.fieldsets.get(*fieldset))
        {
            let span = Span::call_site();
            let name = Ident::new(&reg.name, span);
            let doc = util::doc(&reg.doc.map(ToString::to_string));
            let field_modules = super::fieldset::render(ir, fieldset)?;
            tokens.extend(quote! {
                #doc
                pub mod #name {
                    #field_modules
                }
            });
        }
        Ok(())
    })?;

    block.subblocks().try_for_each(|block| -> Result<()> {
        let block_mod = render_module(block, ir)?;

        let span = Span::call_site();
        let mod_name = Ident::new(&block.module, span);
        tokens.extend(quote! {
            pub mod #mod_name {
                #block_mod
            }
        });
        Ok(())
    })?;

    Ok(tokens)
}

pub fn render(ir: &ir::IR, b: &ir::Block, path: &str) -> Result<TokenStream> {
    let block = Block::new(path, b, ir, None);
    render_module(&block, ir)
}
