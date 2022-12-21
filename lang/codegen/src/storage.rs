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

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    parse2,
    spanned::Spanned,
    Data,
    DataEnum,
    DataStruct,
    DataUnion,
    Field,
    Fields,
};

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
            const KEY: ::ink::primitives::Key = #storage_key;
        }
    })
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

fn generate_struct(s: &synstructure::Structure, struct_item: DataStruct, storage_key: &TokenStream) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let types = s.ast().generics.clone();
    let attrs = s.ast().attrs.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = struct_item
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| convert_into_storage_field(&struct_ident, None, &storage_key, i, field));

    match struct_item.fields {
        Fields::Unnamed(_) => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure (
                    #(#fields),*
                );
            }
        }
        _ => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure {
                    #(#fields),*
                }
            }
        }
    }
}

fn generate_enum(s: &synstructure::Structure, enum_item: DataEnum, storage_key: &TokenStream) -> TokenStream {
    let enum_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let variants = enum_item.variants.into_iter().map(|variant| {
        let attrs = variant.attrs;
        let variant_ident = &variant.ident;
        let discriminant = if let Some((eq, expr)) = variant.discriminant {
            quote! { #eq #expr}
        } else {
            quote! {}
        };

        let fields: Vec<_> = variant
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| convert_into_storage_field(&enum_ident, Some(variant_ident), &storage_key, i, field))
            .collect();

        let fields = match variant.fields {
            Fields::Named(_) => quote! { { #(#fields),* } },
            Fields::Unnamed(_) => quote! { ( #(#fields),* ) },
            Fields::Unit => quote! {},
        };

        quote! {
            #(#attrs)*
            #variant_ident #fields #discriminant
        }
    });

    quote! {
        #(#attrs)*
        #vis enum #enum_ident #types #where_closure {
            #(#variants),*
        }
    }
}

fn generate_union(s: &synstructure::Structure, union_item: DataUnion, storage_key: &TokenStream) -> TokenStream {
    let union_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = union_item
        .fields
        .named
        .iter()
        .enumerate()
        .map(|(i, field)| convert_into_storage_field(&union_ident, None, &storage_key, i, field));

    quote! {
        #(#attrs)*
        #vis union #union_ident #types #where_closure {
            #(#fields),*
        }
    }
}

fn convert_into_storage_field(
    struct_ident: &Ident,
    variant_ident: Option<&syn::Ident>,
    stoarge_key: &TokenStream,
    index: usize,
    field: &Field,
) -> Field {
    let field_name = if let Some(field_ident) = &field.ident {
        field_ident.to_string()
    } else {
        index.to_string()
    };

    let variant_name = if let Some(variant_ident) = variant_ident {
        variant_ident.to_string()
    } else {
        "".to_string()
    };

    let key = ::ink_primitives::KeyComposer::compute_key(
        struct_ident.to_string().as_str(),
        variant_name.as_str(),
        field_name.as_str(),
    )
    .expect("unable to compute the storage key for the field");

    let mut new_field = field.clone();
    let ty = field.ty.clone().to_token_stream();
    let span = field.ty.span();
    let new_ty = syn::Type::Verbatim(quote_spanned!(span =>
        <#ty as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<#key, ::ink::storage::traits::ManualKey<#stoarge_key>>,
        >>::Type
    ));
    new_field.ty = new_ty;
    new_field
}

pub fn upgradeable_storage(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let storage_key = attrs.clone();

    let occupy_storage = occupy_storage_derive(&storage_key, s.clone());
    let storage_key_derived = storage_key_derive(&storage_key, s.clone());
    let storable_hint = storable_hint_derive(&storage_key, s.clone());

    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item, &storage_key),
        Data::Enum(enum_item) => generate_enum(&s, enum_item, &storage_key),
        Data::Union(union_item) => generate_union(&s, union_item, &storage_key),
    };

    let out = quote! {
        #[derive(::ink::storage::traits::Storable)]
        #[cfg_attr(feature = "std", derive(
            ::scale_info::TypeInfo,
            ::ink::storage::traits::StorageLayout
        ))]
        #item

        #storage_key_derived
        #storable_hint

        #occupy_storage
    };

    out.into()
}
