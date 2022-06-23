// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use proc_macro2::TokenStream;
use quote::{
    quote,
    ToTokens,
};

/// Generates the tokens to compute the maximum of the numbers given via
/// their token streams at compilation time.
///
/// # Note
///
/// Since Rust currently does not allow conditionals in const contexts
/// we use the array indexing trick to compute the maximum element:
///
/// ```no_compile
/// max(a, b) = [a, b][(a < b) as usize]
/// ```
fn max_n(args: &[TokenStream]) -> TokenStream {
    match args.split_first() {
        Some((head, rest)) => {
            let rest = max_n(rest);
            quote! {
                [#head, #rest][(#head < #rest) as ::core::primitive::usize]
            }
        }
        None => quote! { 0u64 },
    }
}

/// Generates the tokens for the `SpreadLayout` footprint of some type.
fn footprint(s: &synstructure::Structure) -> TokenStream {
    let variant_footprints = s
        .variants()
        .iter()
        .map(|variant| {
            variant
                .ast()
                .fields
                .iter()
                .map(|field| &field.ty)
                .map(|ty| quote! { <#ty as ::ink_storage::traits::SpreadLayout>::FOOTPRINT })
                .fold(quote! { 0u64 }, |lhs, rhs| {
                    quote! { (#lhs + #rhs) }
                })
        })
        .collect::<Vec<_>>();
    max_n(&variant_footprints[..])
}

/// Generates the tokens for the `SpreadLayout` `REQUIRES_DEEP_CLEAN_UP` constant for the given structure.
fn requires_deep_clean_up(s: &synstructure::Structure) -> TokenStream {
    s.variants()
        .iter()
        .map(|variant| {
            variant
                .ast()
                .fields
                .iter()
                .map(|field| &field.ty)
                .map(|ty| quote! { <#ty as ::ink_storage::traits::SpreadLayout>::REQUIRES_DEEP_CLEAN_UP })
                .fold(quote! { false }, |lhs, rhs| {
                    quote! { (#lhs || #rhs) }
                })
        })
        .fold(quote! { false }, |lhs, rhs| {
            quote! { (#lhs || #rhs) }
        })
}

/// `SpreadLayout` derive implementation for `struct` types.
fn spread_layout_struct_derive(storage_key: &TokenStream, s: &synstructure::Structure) -> TokenStream {
    assert!(s.variants().len() == 1, "can only operate on structs");
    let footprint_body = footprint(s);
    let requires_deep_clean_up_body = requires_deep_clean_up(s);
    let variant: &synstructure::VariantInfo = &s.variants()[0];
    let pull_body = variant.construct(|field, _index| {
        let ty = &field.ty;
        quote! {
            <#ty as ::ink_storage::traits::SpreadLayout>::pull_spread(__key_ptr)
        }
    });
    let push_body = variant.each(|binding| {
        quote! {
            ::ink_storage::traits::SpreadLayout::push_spread(#binding, __key_ptr);
        }
    });
    let clear_body = s.each(|field| {
        quote! {
            ::ink_storage::traits::SpreadLayout::clear_spread(#field, __key_ptr);
        }
    });
    s.gen_impl(quote! {
        gen impl ::ink_storage::traits::SpreadLayout for @Self {
            #[allow(unused_comparisons)]
            const FOOTPRINT: ::core::primitive::u64 = #footprint_body;
            const REQUIRES_DEEP_CLEAN_UP: ::core::primitive::bool = #requires_deep_clean_up_body;

            fn pull_spread(_: &mut ::ink_storage::traits::KeyPtr) -> Self {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );

                if ::ink_env::get_contract_storage::<()>(__key_ptr.key())
                    .expect("could not properly decode storage entry")
                    .is_none()
                {
                    return <Self as ::ink_storage::traits::SpreadAllocate>::allocate_spread(__key_ptr);
                }

                #pull_body
            }
            fn push_spread(&self, _: &mut ::ink_storage::traits::KeyPtr) {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );
                ::ink_env::set_contract_storage_return_size::<()>(__key_ptr.key(), &());
                match self { #push_body }
            }
            fn clear_spread(&self, _: &mut ::ink_storage::traits::KeyPtr) {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );
                match self { #clear_body }
            }
        }
    })
}

/// `SpreadLayout` derive implementation for `enum` types.
fn spread_layout_enum_derive(storage_key: &TokenStream, s: &synstructure::Structure) -> TokenStream {
    assert!(
        !s.variants().is_empty(),
        "encountered invalid empty enum type deriving SpreadLayout trait"
    );
    let footprint_body = footprint(s);
    let requires_deep_clean_up_body = requires_deep_clean_up(s);
    let pull_body = s
        .variants()
        .iter()
        .map(|variant| {
            variant.construct(|field, _index| {
                let ty = &field.ty;
                quote! {
                    <#ty as ::ink_storage::traits::SpreadLayout>::pull_spread(__key_ptr)
                }
            })
        })
        .enumerate()
        .fold(quote! {}, |acc, (index, variant)| {
            let index = index as u8;
            quote! {
                #acc
                #index => #variant,
            }
        });

    let push_body = s.variants().iter().enumerate().map(|(index, variant)| {
        let pat = variant.pat();
        let index = index as u8;
        let fields = variant.bindings().iter().map(|field| {
            quote! {
                ::ink_storage::traits::SpreadLayout::push_spread(#field, __key_ptr);
            }
        });
        quote! {
            #pat => {
                { <::core::primitive::u8 as ::ink_storage::traits::SpreadLayout>::push_spread(&#index, __key_ptr); }
                #(
                    { #fields }
                )*
            }
        }
    });
    let clear_body = s.each(|field| {
        quote! {
            ::ink_storage::traits::SpreadLayout::clear_spread(#field, __key_ptr);
        }
    });
    s.gen_impl(quote! {
        gen impl ::ink_storage::traits::SpreadLayout for @Self {
            #[allow(unused_comparisons)]
            const FOOTPRINT: ::core::primitive::u64 = 1 + #footprint_body;

            const REQUIRES_DEEP_CLEAN_UP: ::core::primitive::bool = #requires_deep_clean_up_body;

            fn pull_spread(_: &mut ::ink_storage::traits::KeyPtr) -> Self {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );

                if ::ink_env::get_contract_storage::<::core::primitive::u8>(__key_ptr.key())
                    .expect("could not properly decode storage entry")
                    .is_none()
                {
                    return <Self as ::ink_storage::traits::SpreadAllocate>::allocate_spread(__key_ptr);
                }

                match <::core::primitive::u8 as ::ink_storage::traits::SpreadLayout>::pull_spread(__key_ptr) {
                    #pull_body
                    _ => unreachable!("encountered invalid enum discriminant"),
                }
            }
            fn push_spread(&self, _: &mut ::ink_storage::traits::KeyPtr) {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );
                match self {
                    #(
                        #push_body
                    )*
                }
            }
            fn clear_spread(&self, _: &mut ::ink_storage::traits::KeyPtr) {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );
                match self {
                    #clear_body
                }
            }
        }
    })
}

/// Derives `ink_storage`'s `SpreadAllocate` trait for the given type.
pub fn spread_allocate_derive(storage_key: &TokenStream, mut s: synstructure::Structure) -> TokenStream {
    s.bind_with(|_| synstructure::BindStyle::Move)
        .add_bounds(synstructure::AddBounds::Generics)
        .underscore_const(true);
    match s.ast().data {
        syn::Data::Struct(_) => derive_struct(storage_key, s),
        syn::Data::Enum(_) => {
            panic!("cannot derive `SpreadAllocate` for `enum` types")
        }
        syn::Data::Union(_) => {
            panic!("cannot derive `SpreadAllocate` for `union` types")
        }
    }
}

/// Derives `ink_storage`'s `SpreadAllocate` trait for the given `struct`.
fn derive_struct(storage_key: &TokenStream, s: synstructure::Structure) -> TokenStream {
    assert!(s.variants().len() == 1, "can only operate on structs");
    let variant = &s.variants()[0];
    let allocate_body = variant.construct(|field, _index| {
        let ty = &field.ty;
        quote! {
            <#ty as ::ink_storage::traits::SpreadAllocate>::allocate_spread(__key_ptr)
        }
    });
    s.gen_impl(quote! {
        gen impl ::ink_storage::traits::SpreadAllocate for @Self {
            #[inline(never)]
            fn allocate_spread(_: &mut ::ink_primitives::KeyPtr) -> Self {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );
                #allocate_body
            }
        }
    })
}

pub fn spread_layout_derive(storage_key: &TokenStream, mut s: synstructure::Structure) -> TokenStream {
    s.bind_with(|_| synstructure::BindStyle::Move)
        .add_bounds(synstructure::AddBounds::None)
        .underscore_const(true);
    match s.ast().data {
        syn::Data::Struct(_) => spread_layout_struct_derive(&storage_key, &s),
        syn::Data::Enum(_) => spread_layout_enum_derive(&storage_key, &s),
        _ => {
            panic!("cannot derive `SpreadLayout` or `PackedLayout` for Rust `union` items")
        }
    }
}

fn field_layout<'a>(variant: &'a synstructure::VariantInfo) -> impl Iterator<Item = TokenStream> + 'a {
    variant.ast().fields.iter().map(|field| {
        let ident = match field.ident.as_ref() {
            Some(ident) => {
                let ident_str = ident.to_string();
                quote! { ::core::option::Option::Some(#ident_str) }
            }
            None => quote! { ::core::option::Option::None },
        };
        let ty = &field.ty;
        quote! {
            ::ink_metadata::layout::FieldLayout::new(
                #ident,
                <#ty as ::ink_storage::traits::StorageLayout>::layout(__key_ptr),
            )
        }
    })
}

fn storage_layout_struct(storage_key: &TokenStream, s: &synstructure::Structure) -> TokenStream {
    assert!(matches!(s.ast().data, syn::Data::Struct(_)), "s must be a struct item");
    assert!(s.variants().len() == 1, "structs must have at most one variant");
    let variant: &synstructure::VariantInfo = &s.variants()[0];
    let field_layouts = field_layout(variant);
    s.gen_impl(quote! {
        gen impl ::ink_storage::traits::StorageLayout for @Self {
            fn layout(_: &mut ::ink_storage::traits::KeyPtr) -> ::ink_metadata::layout::Layout {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );
                ::ink_metadata::layout::Layout::Struct(
                    ::ink_metadata::layout::StructLayout::new([
                        #(#field_layouts ,)*
                    ])
                )
            }
        }
    })
}

fn storage_layout_enum(storage_key: &TokenStream, s: &synstructure::Structure) -> TokenStream {
    assert!(matches!(s.ast().data, syn::Data::Enum(_)), "s must be an enum item");
    let variant_layouts = s.variants().iter().enumerate().map(|(n, variant)| {
        let discriminant = variant
            .ast()
            .discriminant
            .as_ref()
            .map(|(_, expr)| quote! { #expr })
            .unwrap_or_else(|| quote! { #n });
        let field_layouts = field_layout(variant);
        quote! {
            {
                let mut __variant_key_ptr = *__key_ptr;
                let mut __key_ptr = &mut __variant_key_ptr;
                (
                    ::ink_metadata::layout::Discriminant::from(#discriminant),
                    ::ink_metadata::layout::StructLayout::new([
                        #(#field_layouts ,)*
                    ]),
                )
            }
        }
    });
    s.gen_impl(quote! {
        gen impl ::ink_storage::traits::StorageLayout for @Self {
            fn layout(_: &mut ::ink_storage::traits::KeyPtr) -> ::ink_metadata::layout::Layout {
                let __key_ptr = &mut ::ink_storage::traits::KeyPtr::from(
                    ::ink_primitives::Key::from(#storage_key)
                );

                let dispatch_key = __key_ptr.advance_by(1);
                ::ink_metadata::layout::Layout::Enum(
                    ::ink_metadata::layout::EnumLayout::new(
                        ::ink_metadata::layout::LayoutKey::from(dispatch_key),
                        [
                            #(#variant_layouts ,)*
                        ]
                    )
                )
            }
        }
    })
}

pub fn storage_layout_derive(storage_key: &TokenStream, mut s: synstructure::Structure) -> TokenStream {
    s.bind_with(|_| synstructure::BindStyle::Move)
        .add_bounds(synstructure::AddBounds::Generics)
        .underscore_const(true);
    match s.ast().data {
        syn::Data::Struct(_) => storage_layout_struct(storage_key, &s),
        syn::Data::Enum(_) => storage_layout_enum(storage_key, &s),
        _ => panic!("cannot derive `StorageLayout` for Rust `union` items"),
    }
}

pub fn storage(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let storage_key = attrs;

    let spread_layout = spread_layout_derive(&storage_key, s.clone());
    let spread_allocate = spread_allocate_derive(&storage_key, s.clone());
    let storage_layout = storage_layout_derive(&storage_key, s.clone());
    let item = s.ast().to_token_stream();

    (quote! {
        #item

        #spread_layout
        #spread_allocate
        #[cfg(feature = "std")]
        #storage_layout
    })
    .into()
}
