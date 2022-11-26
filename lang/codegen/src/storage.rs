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
use syn::spanned::Spanned;

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

fn field_layout<'a>(variant: &'a synstructure::VariantInfo) -> impl Iterator<Item = TokenStream> + 'a {
    variant.ast().fields.iter().enumerate().map(|i, field| {
        let ident = match field.ident.as_ref() {
            Some(ident) => {
                let ident_str = ident.to_string();
                quote! { #ident_str }
            }
            None => {
                let index = i.to_string();
                quote! { #index }
            }
        };
        let ty = &field.ty;
        quote! {
            ::ink::metadata::layout::FieldLayout::new(
                #ident,
                <#ty as ::ink_storage::traits::StorageLayout>::layout(__key),
            )
        }
    })
}

fn storage_layout_struct(storage_key: &TokenStream, s: &synstructure::Structure) -> TokenStream {
    assert!(matches!(s.ast().data, syn::Data::Struct(_)), "s must be a struct item");
    assert!(s.variants().len() == 1, "structs must have at most one variant");
    let struct_ident = s.ast().ident.clone();
    let variant: &synstructure::VariantInfo = &s.variants()[0];
    let field_layouts = field_layout(variant);
    s.gen_impl(quote! {
        gen impl ::ink::storage::traits::StorageLayout for @Self {
            fn layout(__key: &::ink::primitives::Key) -> ::ink_metadata::layout::Layout {
                let __key = &mut ::ink::primitives::Key::from(#storage_key)
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        ::core::stringify!(#struct_ident),
                        [
                            #(#field_layouts ,)*
                        ]
                    )
                )
            }
        }
    })
}

fn storage_layout_enum(storage_key: &TokenStream, s: &synstructure::Structure) -> TokenStream {
    assert!(matches!(s.ast().data, syn::Data::Enum(_)), "s must be an enum item");

    if s.variants().len() > 256 {
        return syn::Error::new(
            s.ast().span(),
            "Currently only enums with at most 256 variants are supported.",
        )
        .to_compile_error()
    }

    let variant_layouts = s.variants().iter().enumerate().map(|(n, variant)| {
        let variant_ident = variant.ast().ident;
        let discriminant = variant
            .ast()
            .discriminant
            .as_ref()
            .map(|(_, expr)| quote! { #expr })
            .unwrap_or_else(|| quote! { #n });
        let field_layouts = field_layout(variant);
        quote! {
            {
                (
                    ::ink::metadata::layout::Discriminant::from(#discriminant),
                    ::ink_metadata::layout::StructLayout::new(
                        ::core::stringify!(#variant_ident),
                        [
                            #(#field_layouts ,)*
                        ]
                    ),
                )
            }
        }
    });
    let enum_ident = s.ast().ident.clone();
    s.gen_impl(quote! {
        gen impl ::ink::storage::traits::StorageLayout for @Self {
            fn layout(_: &mut ::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                let __key = &mut ::ink::primitives::Key::from(#storage_key)
                ::ink::metadata::layout::Layout::Enum(
                    ::ink::metadata::layout::EnumLayout::new(
                        ::core::stringify!(#enum_ident),
                        ::ink_metadata::layout::LayoutKey::from(__key),
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
        .add_bounds(synstructure::AddBounds::Fields)
        .underscore_const(true);
    match &s.ast().data {
        syn::Data::Struct(_) => storage_layout_struct(storage_key, &s),
        syn::Data::Enum(_) => storage_layout_enum(storage_key, &s),
        _ => panic!("cannot derive `StorageLayout` for Rust `union` items"),
    }
}

pub fn occupy_storage_derive(storage_key: &TokenStream, mut s: synstructure::Structure) -> TokenStream {
    s.add_bounds(synstructure::AddBounds::None).underscore_const(true);
    let occupy_storage = s.gen_impl(quote! {
        gen impl ::openbrush::traits::OccupyStorage for @Self {
            const KEY: ::core::primitive::u32 = #storage_key;
        }
    });
    let storage = s.gen_impl(quote! {
        gen impl ::openbrush::traits::Storage<Self> for @Self {
            fn get(&self) -> &Self {
                self
            }

            fn get_mut(&mut self) -> &mut Self {
                self
            }
        }
    });
    let occupied_storage = s.gen_impl(quote! {
        gen impl ::openbrush::traits::OccupiedStorage<{ #storage_key }> for @Self {
            type WithData = Self;
        }
    });

    quote! {
        #occupy_storage
        #storage
        #occupied_storage
    }
}

pub fn upgradeable_storage(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let storage_key_u32 = attrs.clone();
    let storage_key = quote! {
        ::openbrush::utils::StorageKeyConvertor::old_key(#attrs)
    };

    let storage_layout = storage_layout_derive(&storage_key, s.clone());
    let occupy_storage = occupy_storage_derive(&storage_key_u32, s.clone());
    let item = s.ast().to_token_stream();

    (quote! {
        #item

        #[cfg(feature = "std")]
        #storage_layout

        #occupy_storage
    })
    .into()
}
