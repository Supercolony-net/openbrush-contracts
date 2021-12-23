#![cfg_attr(not(feature = "std"), no_std)]

/// This `macro_rule` generates a procedural derive macro for storage trait.
///
/// The first argument is the name of the procedural function.
/// The second argument is the name of the trait for which derive will be generated.
/// The third argument is the name of the marker for the derive macro. This marker specifies
/// for derive macro which field will be returned by the implementation for the storage trait.
///
/// An example of the usage of this macro can be found in `derive` folder of any contract
/// implemented by this library. For example [OwnableStorage](ownable_derive::OwnableStorage).
#[macro_export]
macro_rules! declare_derive_storage_trait {
    ($derive_name:ident,$trait_name:ident,$trait_field_specifier:ident) => {
        #[proc_macro_derive($trait_name, attributes($trait_field_specifier))]
        pub fn $derive_name(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let derive = syn::parse_macro_input!(_item as syn::DeriveInput);
            const TRAIT_NAME: &'static str = stringify!($trait_name);
            const FIELD_SETTER: &'static str = stringify!($trait_field_specifier);

            let struct_ident = derive.ident;

            let field_ident;
            let field_ty;
            if let syn::Data::Struct(data) = &derive.data {
                if let syn::Fields::Named(named_fields) = &data.fields {
                    let field = named_fields
                        .named
                        .iter()
                        .find(|f| f.attrs.iter().find(|a| a.path.is_ident(FIELD_SETTER)).is_some());

                    if let Some(field) = field {
                        field_ident = field.ident.clone();
                        field_ty = field.ty.clone();
                    } else {
                        let err_message = format!("Struct doesn't specify {} for trait {}", FIELD_SETTER, TRAIT_NAME);
                        return quote::quote! {
                            #[cfg(not(feature = "ink-as-dependency"))]
                            compile_error!(#err_message);
                        }
                        .into()
                    }
                } else {
                    let err_message = format!("{} only supports named fields in struct", FIELD_SETTER);
                    return quote::quote! {
                        #[cfg(not(feature = "ink-as-dependency"))]
                        compile_error!(#err_message);
                    }
                    .into()
                }
            } else {
                let err_message = format!("{} only supports struct", FIELD_SETTER);
                return quote::quote! {
                    #[cfg(not(feature = "ink-as-dependency"))]
                    compile_error!(#err_message);
                }
                .into()
            }

            let code = quote::quote! {
                impl $trait_name for #struct_ident {
                    fn get(&self) -> & #field_ty {
                        &self.#field_ident
                    }

                    fn get_mut(&mut self) -> &mut #field_ty {
                        &mut self.#field_ident
                    }
                }
            };
            code.into()
        }
    };
}
