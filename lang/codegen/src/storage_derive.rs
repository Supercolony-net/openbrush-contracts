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
use quote::quote;
use syn::{
    parse2,
    spanned::Spanned,
    Data,
};

pub fn storage_derive(item: TokenStream) -> TokenStream {
    let derive: syn::DeriveInput = parse2(item).expect("Expected DeriveInput");

    let struct_ident = derive.ident;
    let (impls, types, where_clause) = derive.generics.split_for_impl();

    let fields: Vec<_> = match &derive.data {
        Data::Struct(st) => st.fields.iter().collect(),
        Data::Enum(en) => en.variants.iter().flat_map(|v| v.fields.iter()).collect(),
        Data::Union(un) => un.fields.named.iter().collect(),
    };

    let impls = fields
        .iter()
        .filter(|field| field.attrs.iter().find(|a| a.path.is_ident("storage_field")).is_some())
        .map(|field| {
            let field_ident = field.ident.clone();
            let ty = field.ty.clone();
            let span = field.span();

            quote::quote_spanned!(span=>
                impl #impls ::openbrush::traits::Storage<#ty> for #struct_ident #types #where_clause {
                    fn get(&self) -> &#ty {
                        &self.#field_ident
                    }

                    fn get_mut(&mut self) -> &mut #ty {
                        &mut self.#field_ident
                    }
                }

                impl #impls ::openbrush::traits::OccupiedStorage<{ <#ty as ::openbrush::traits::OccupyStorage>::KEY }>
                    for #struct_ident #types #where_clause
                {
                    type WithData = #ty;
                }
            )
        });

    (quote! {
        #(#impls)*
    })
    .into()
}
