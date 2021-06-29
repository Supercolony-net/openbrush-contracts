extern crate proc_macro;
mod internal;
mod contract;
mod trait_definition;
mod storage_trait;

use quote::{
    quote,
};
use syn::{
    ItemFn,
    TraitItemMethod,
};
use proc_macro::{TokenStream};
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};

#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    contract::generate(_attrs, ink_module)
}

/// Marks trait definition to brush as special ink trait definition.
/// It is the same ink trait definition, but with additional features:
/// - Allows use super trait
/// - Allows definition a default implementation of methods
/// - Allows to have internal functions(without `#[ink(message)]`)
/// - Allows call implementation from trait during overriding(via #[super] + method)
///
/// This macro stores definition of the trait to temporary file during build process.
/// Based on this definition [`#[brush::contract]`](`macro@crate::contract`)
/// will generate implementation of this trait. If you defined a default implementation,
/// [`#[brush::contract]`](`macro@crate::contract`) will copy the default implementation from the trait
/// and will paste it in impl section. It means that your default implementation must be public
/// and exported as a part of crate.
///
///  ** Note ** You don't need to copy/paste attributes from trait definition, it will be done automatically
///  ** Note ** Super trait is not used during build process, it is only syntactic sugar for your IDE
///  ** Note ** Internal methods are not stored in trait, they will be extracted to separate impl section
/// of your struct, so their implementation also must be a public
///  ** Note ** This macro must be processed before [`#[brush::contract]`](`macro@crate::contract`),
/// otherwise it will fail
///
/// # Example: Definition
///
/// ```
/// pub use ink_storage::{
///     collections::{
///         HashMap as StorageHashMap,
///     },
/// };
/// use brush::traits::{AccountId, Balance};
///
/// #[brush::trait_definition]
/// pub trait PSP20: PSP20Storage {
///     /// Returns the account Balance for the specified `owner`.
///     #[ink(message)]
///     fn balance_of(&self, owner: AccountId) -> Balance {
///         self._balances().get(&owner).copied().unwrap_or(0)
///     }
///
///     /// Transfers `value` amount of tokens from the caller's account to account `to`.
///     #[ink(message)]
///     fn transfer(&mut self, to: AccountId, value: Balance) {
///         let from = Self::env().caller();
///         self._transfer_from_to(from, to, value)
///     }
///
///     fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///         let from_balance = self.balance_of(from);
///         assert!(from_balance >= amount, "InsufficientBalance");
///         self._balances_mut().insert(from, from_balance - amount);
///         let to_balance = self.balance_of(to);
///         self._balances_mut().insert(to, to_balance + amount);
///     }
/// }
/// ```
///
/// # Example: Implementation
///
/// It uses storage trait from above.
///
/// ```
/// #[brush::contract]
/// mod base_psp20 {
///     pub use ink_storage::collections::{HashMap as StorageHashMap};
///
///     #[brush::storage_trait]
///     pub trait PSP20ExampleStorage {
///         fn _supply(&self) -> & Balance;
///         fn _supply_mut(&mut self) -> &mut Balance;
///
///         fn _balances(&self) -> & StorageHashMap<AccountId, Balance>;
///         fn _balances_mut(&mut self) -> &mut StorageHashMap<AccountId, Balance>;
///
///         fn _allowances(&self) -> & StorageHashMap<(AccountId, AccountId), Balance>;
///         fn _allowances_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), Balance>;
///     }
///
///     #[brush::trait_definition]
///     pub trait PSP20Example: PSP20ExampleStorage {
///         /// Returns the account Balance for the specified `owner`.
///         #[ink(message)]
///         fn balance_of(&self, owner: AccountId) -> Balance {
///             self._balances().get(&owner).copied().unwrap_or(0)
///         }
///
///         /// Transfers `value` amount of tokens from the caller's account to account `to`.
///         #[ink(message)]
///         fn transfer(&mut self, to: AccountId, value: Balance) {
///             let from = Self::env().caller();
///             self._transfer_from_to(from, to, value)
///         }
///
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             let from_balance = self.balance_of(from);
///             assert!(from_balance >= amount, "InsufficientBalance");
///             self._balances_mut().insert(from, from_balance - amount);
///             let to_balance = self.balance_of(to);
///             self._balances_mut().insert(to, to_balance + amount);
///         }
///     }
///
///     #[ink(storage)]
///     #[derive(Default, PSP20ExampleStorage)]
///     pub struct PSP20Struct {
///         hated_account: AccountId,
///     }
///
///     impl PSP20Example for PSP20Struct {
///         // Let's override method to reject transactions to bad account
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             assert!(to != self.hated_account, "I hate this account!");
///             #[super]self._transfer_from_to(from, to, amount);
///         }
///     }
///
///     impl PSP20Struct {
///         #[ink(constructor)]
///         pub fn new(hated_account: AccountId) -> Self {
///             let mut instance = Self::default();
///             instance.hated_account = hated_account;
///             instance
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn trait_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    trait_definition::generate(_attrs, _input)
}

/// Marks trait definition to brush as special storage trait definition.
///
/// This macro stores definition of the trait to temporary file during build process.
/// Based on this definition [`#[brush::contract]`](`macro@crate::contract`)
/// will generate fields and getters for struct, which will derive this trait.
/// The name of each field is substring between prefix `_` and suffix `_mut`.
/// The type of the field is the return type of getter(It means that you need to use
/// the same naming of types in the crate where you will derive this trait).
///
/// There are some restrictions that you must follow:
/// - The trait marked by this macro must contain the only definition of getters
///   for fields of some structure(no other logic).
/// - The first character of the name of method must be `_` underscore.
/// - Each fields must contains two getter(no more, no less):
///   - Getter by reference `_field() -> & Type`
///   - Getter by mut reference `_field_mut() -> &mut Type`
/// - The getter by mut reference must have the same name as the getter by reference + suffix `_mut`.
///
///  ** Note ** This macro must be processed before [`#[brush::contract]`](`macro@crate::contract`),
/// otherwise it will fail
///
/// # Example: Definition
///
/// ```
/// pub use ink_storage::{
///     collections::{
///         HashMap as StorageHashMap,
///     },
/// };
/// use brush::traits::{AccountId, Balance};
///
/// #[brush::storage_trait]
/// pub trait PSP20ExampleStorage {
///     fn _supply(&self) -> & Balance;
///     fn _supply_mut(&mut self) -> &mut Balance;
///
///     fn _balances(&self) -> & StorageHashMap<AccountId, Balance>;
///     fn _balances_mut(&mut self) -> &mut StorageHashMap<AccountId, Balance>;
///
///     fn _allowances(&self) -> & StorageHashMap<(AccountId, AccountId), Balance>;
///     fn _allowances_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), Balance>;
/// }
/// ```
///
/// # Example: Implementation
///
/// It uses storage trait from above.
///
/// ```
/// #[brush::contract]
/// mod base_psp20 {
///     pub use ink_storage::collections::{HashMap as StorageHashMap};
///
///     #[brush::storage_trait]
///     pub trait PSP20ExampleStorage {
///         fn _supply(&self) -> & Balance;
///         fn _supply_mut(&mut self) -> &mut Balance;
///
///         fn _balances(&self) -> & StorageHashMap<AccountId, Balance>;
///         fn _balances_mut(&mut self) -> &mut StorageHashMap<AccountId, Balance>;
///
///         fn _allowances(&self) -> & StorageHashMap<(AccountId, AccountId), Balance>;
///         fn _allowances_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), Balance>;
///     }
///
///     #[ink(storage)]
///     #[derive(Default, PSP20ExampleStorage)]
///     pub struct PSP20Struct {}
///
///     impl PSP20Struct {
///         #[ink(constructor)]
///         pub fn new(initial_supply: Balance) -> Self {
///             let mut instance = Self::default();
///             *instance._supply_mut() = initial_supply;
///             instance
///         }
///
///         /// Returns the total supply of the smart contract.
///         #[ink(message)]
///         pub fn total_supply(&self) -> Balance {
///             self._supply().clone()
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn storage_trait(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    storage_trait::generate(_attrs, _input)
}

#[proc_macro_attribute]
pub fn modifiers(_attrs: TokenStream, method: TokenStream) -> TokenStream {
    let attrs: TokenStream2 = _attrs.into();
    let modifiers = attrs
        .into_iter()
        .filter_map(|token|
            if let TokenTree::Ident(ident) = token {
                Some(ident)
            } else {
                None
            })
        .collect();

    let fn_item = syn::parse2::<ItemFn>(method.clone().into());
    let trait_method_item = syn::parse2::<TraitItemMethod>(method.clone().into());

    let mut code: TokenStream2 = method.into();
    if let Ok(mut item) = fn_item {
        add_modifiers_to_block(&mut item.block, modifiers);
        code = quote! { #item };
    } else if let Ok(mut item) = trait_method_item {
        if let Some(block) = &mut item.default {
            add_modifiers_to_block(block, modifiers);
            code = quote! { #item };
        }
    }

    code.into()
}

#[inline]
fn add_modifiers_to_block(block: &mut syn::Block, modifiers: Vec<syn::Ident>) {
    modifiers
        .into_iter()
        .for_each(|ident| {
            let code = quote! {
                #[cfg(not(feature = "ink-as-dependency"))] self.#ident();
            };
            block.stmts.insert(0, syn::parse2::<syn::Stmt>(code)
                .expect("Can't parse statement of modifier"));
        });
}