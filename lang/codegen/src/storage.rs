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
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    parse2,
    spanned::Spanned,
};

fn field_layout<'a>(variant: &'a synstructure::VariantInfo) -> impl Iterator<Item = TokenStream> + 'a {
    variant.ast().fields.iter().enumerate().map(|(i, field)| {
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
                <#ty as ::ink::storage::traits::StorageLayout>::layout(__key),
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
            fn layout(_: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                let __key = &mut ::ink::primitives::Key::from(#storage_key);
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
                    ::ink::metadata::layout::StructLayout::new(
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
            fn layout(_: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                let __key = &mut ::ink::primitives::Key::from(#storage_key);
                ::ink::metadata::layout::Layout::Enum(
                    ::ink::metadata::layout::EnumLayout::new(
                        ::core::stringify!(#enum_ident),
                        ::ink::metadata::lay    out::LayoutKey::from(__key),
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

pub fn storage_key_derive(storage_key: &TokenStream, mut s: synstructure::Structure) -> TokenStream {
    s.add_bounds(synstructure::AddBounds::None).underscore_const(true);

    s.gen_impl(quote! {
        gen impl ::ink::storage::traits::StorageKey for @Self {
            const KEY: ::ink::primitives::Key = ::ink::primitives::Key::from(#storage_key);
        }
    })
}

fn storable_struct_derive(s: &synstructure::Structure) -> TokenStream {
    assert_eq!(s.variants().len(), 1, "can only operate on structs");
    let variant: &synstructure::VariantInfo = &s.variants()[0];
    let decode_body = variant.construct(|field, _index| {
        let ty = &field.ty;
        let span = ty.span();
        quote_spanned!(span =>
            <#ty as ::ink::storage::traits::Storable>::decode(__input)?
        )
    });
    let encode_body = variant.each(|binding| {
        let span = binding.ast().ty.span();
        quote_spanned!(span =>
            ::ink::storage::traits::Storable::encode(#binding, __dest);
        )
    });

    s.gen_impl(quote! {
        gen impl ::ink::storage::traits::Storable for @Self {
           #[inline(always)]
           #[allow(non_camel_case_types)]
           fn decode<__ink_I: ::scale::Input>(__input: &mut __ink_I) -> ::core::result::Result<Self, ::scale::Error> {
               ::core::result::Result::Ok(#decode_body)
           }

           #[inline(always)]
           #[allow(non_camel_case_types)]
           fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(&self, __dest: &mut __ink_O) {
               match self { #encode_body }
           }
        }
    })
}

/// `Storable` derive implementation for `enum` types.
fn storable_enum_derive(s: &synstructure::Structure) -> TokenStream {
    assert!(
        !s.variants().is_empty(),
        "encountered invalid empty enum type deriving Storable trait"
    );

    if s.variants().len() > 256 {
        return syn::Error::new(
            s.ast().span(),
            "Currently only enums with at most 256 variants are supported.",
        )
        .to_compile_error()
    }

    let decode_body = s
        .variants()
        .iter()
        .map(|variant| {
            variant.construct(|field, _index| {
                let ty = &field.ty;
                let span = ty.span();
                quote_spanned!(span =>
                    <#ty as ::ink::storage::traits::Storable>::decode(__input)?
                )
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

    let encode_body = s.variants().iter().enumerate().map(|(index, variant)| {
        let pat = variant.pat();
        let index = index as u8;
        let fields = variant.bindings().iter().map(|field| {
            let span = field.ast().ty.span();
            quote_spanned!(span =>
                ::ink::storage::traits::Storable::encode(#field, __dest);
            )
        });
        quote! {
            #pat => {
                { <::core::primitive::u8 as ::ink::storage::traits::Storable>::encode(&#index, __dest); }
                #(
                    { #fields }
                )*
            }
        }
    });
    s.gen_impl(quote! {
        gen impl ::ink::storage::traits::Storable for @Self {
           #[inline(always)]
           #[allow(non_camel_case_types)]
           fn decode<__ink_I: ::scale::Input>(__input: &mut __ink_I) -> ::core::result::Result<Self, ::scale::Error> {
               ::core::result::Result::Ok(
                   match <::core::primitive::u8 as ::ink::storage::traits::Storable>::decode(__input)? {
                       #decode_body
                       _ => unreachable!("encountered invalid enum discriminant"),
                   }
               )
           }

           #[inline(always)]
           #[allow(non_camel_case_types)]
           fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(&self, __dest: &mut __ink_O) {
               match self {
                   #(
                       #encode_body
                   )*
               }
           }
        }
    })
}

/// Derives `ink_storage`'s `Storable` trait for the given `struct` or `enum`.
pub fn storable_derive(mut s: synstructure::Structure) -> TokenStream {
    s.bind_with(|_| synstructure::BindStyle::Move)
        .add_bounds(synstructure::AddBounds::Fields)
        .underscore_const(true);
    match &s.ast().data {
        syn::Data::Struct(_) => storable_struct_derive(&s),
        syn::Data::Enum(_) => storable_enum_derive(&s),
        _ => {
            panic!("cannot derive `Storable` for Rust `union` items")
        }
    }
}

fn storable_hint_inner(storage_key: &TokenStream, s: synstructure::Structure) -> TokenStream {
    let ident = s.ast().ident.clone();
    let salt_ident = format_ident!("__ink_generic_salt");

    let mut generics = s.ast().generics.clone();
    generics
        .params
        .push(parse2(quote! { #salt_ident : ::ink::storage::traits::StorageKey }).unwrap());

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let (_, ty_generics_original, _) = s.ast().generics.split_for_impl();

    quote! {
        impl #impl_generics ::ink::storage::traits::StorableHint<#salt_ident> for #ident #ty_generics_original #where_clause {
            type Type = #ident #ty_generics_original;
            type PreferredKey = ::ink::storage::traits::ManualKey<#storage_key>;
        }
    }
}

pub fn storable_hint_derive(storage_key: &TokenStream, s: synstructure::Structure) -> TokenStream {
    let derive = storable_hint_inner(storage_key, s);

    quote! {
        const _ : () = {
            #derive
        };
    }
}

pub fn upgradeable_storage(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let storage_key = attrs.clone();

    let storage_layout = storage_layout_derive(&storage_key, s.clone());
    let occupy_storage = occupy_storage_derive(&storage_key, s.clone());
    let storage_key_derived = storage_key_derive(&storage_key, s.clone());
    let storable_hint = storable_hint_derive(&storage_key, s.clone());
    let storable = storable_derive(s.clone());
    let item = s.ast().to_token_stream();

    let out = quote! {
        #item

        #storage_key_derived
        #storable_hint
        #storable

        #[cfg(feature = "std")]
        #storage_layout

        #occupy_storage
    };

    // println!("{}", out);
    out.into()
}
