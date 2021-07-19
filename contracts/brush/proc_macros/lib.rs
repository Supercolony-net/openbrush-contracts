extern crate proc_macro;
mod internal;
mod contract;
mod trait_definition;
mod storage_trait;
mod metadata;
mod modifier_definition;
mod modifiers;

use proc_macro::TokenStream;

/// Entry point for use brush's macros in ink! smart contracts.
///
/// # Description
///
/// The macro consumes brush's macros to simplify the usage of the library.
/// After consumption, it pastes ink! code and then ink!'s macros will be processed.
///
/// First of all, the macro will process:
/// [`#[brush::storage_trait]`](`macro@crate::storage_trait`),
/// [`#[brush::trait_definition]`](`macro@crate::trait_definition`),
/// [`#[brush::modifier_definition]`](`macro@crate::modifier_definition`).
///
/// After that it will consume every usage of:
/// - Derive of storage trait([`#[brush::storage_trait]`](`macro@crate::storage_trait`)).
/// - Impl of external trait([`#[brush::trait_definition]`](`macro@crate::trait_definition`)).
#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    contract::generate(_attrs, ink_module)
}

/// Defines extensible trait in the scope of brush::contract.
/// It is the same ink trait definition, but with additional features:
/// - Allows using super traits.
/// - Allows defining default implementations of methods.
/// - Allows having internal functions(without `#[ink(message)]`).
/// - Allows calling implementation from trait when overriding (via `#[super] self.transfer( ... )`).
///
/// This macro stores definition of the trait in a temporary file during build process.
/// Based on this definition [`#[brush::contract]`](`macro@crate::contract`)
/// will generate implementation of this trait. If you defined a default implementation,
/// [`#[brush::contract]`](`macro@crate::contract`) will copy the default implementation from the trait
/// and will paste it in impl section. It means that your default implementation must be public
/// and exported as a part of crate.
///
///  ** Note ** You don't need to copy/paste attributes from trait definition, it will be done automatically.
///  ** Note ** Super trait is not used during build process, it is only syntactic sugar for your IDE.
///  ** Note ** Internal methods are not stored in trait, they will be extracted to separate impl section.
/// of your struct, so their implementation also must be public.
///  ** Note ** This macro must be processed before [`#[brush::contract]`](`macro@crate::contract`),
/// otherwise it will fail: It means that [`#[brush::trait_definition]`] must be defined in scope of
/// [`#[brush::contract]`](`macro@crate::contract`)
/// or it must be defined in another crate(macros in dependencies will be processed early).
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
/// pub trait PSP20Storage {
///     fn _balances(&self) -> & StorageHashMap<AccountId, Balance>;
///     fn _balances_mut(&mut self) -> &mut StorageHashMap<AccountId, Balance>;
/// }
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
///         self._transfer_from_to(to, to, value)
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
/// This macro stores definition of the trait in a temporary file during build process.
/// Based on this definition [`#[brush::contract]`](`macro@crate::contract`)
/// will generate fields and getters for struct, which will derive this trait.
/// The name of each field is substring between prefix `_` and suffix `_mut`
/// (e.g. given `_method_name_mut` function, field `method_name` will be generated).
/// The type of the field is the return type of getter (It means that you need to use
/// the same naming of types in the crate where you will derive this trait).
///
/// There are some restrictions that you must follow:
/// - The trait marked by this macro must contain only the definition of getters
///   for fields of some structure (no other logic).
/// - The first character of the name of method must be `_` underscore.
/// - Each field must contain **exactly** two getters:
///   - Getter by reference `_field() -> & Type`
///   - Getter by mut reference `_field_mut() -> &mut Type`
/// - The getter by mut reference must have the same name as the getter by reference + suffix `_mut`.
///
///  ** Note ** This macro must be processed before [`#[brush::contract]`](`macro@crate::contract`),
/// otherwise it will fail: It means that [`#[brush::trait_definition]`] must be defined in scope of
/// [`#[brush::contract]`](`macro@crate::contract`)
/// or it must be defined in another crate(macros in dependencies will be processed early).
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

/// This macro only checks that some free-standing function satisfies a set of rules.
///
/// Rules:
/// - First argument should not be `self`.
/// - First argument must be a reference to a type `instance: &T`. In most cases it's the instance of contract.
/// - Second argument is function's body(this function contains the main code of method attached to the modifier).
/// The type must be `Fn(&T)` or `FnMut(&T)`.
/// - Every next argument should not be references to object.
/// Because modifier allows only to pass arguments by value(Modifier will pass the clone of argument).
/// - The return type of body function(second argument) must be the same as the return type of modifier.
///
/// # Example: Definition
///
/// ```
/// #[derive(Default)]
/// struct Contract {
///     initialized: bool,
/// }
///
/// #[brush::modifier_definition]
/// fn once<BodyFn: Fn(&mut Contract)>(instance: &mut Contract, body: BodyFn, _example_data: u8) {
///     assert!(!instance.initialized, "Contract is already initialized");
///     body(instance);
///     instance.initialized = true;
/// }
/// ```
#[proc_macro_attribute]
pub fn modifier_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    modifier_definition::generate(_attrs, _input)
}

/// Macro calls every modifier function by passing self and the code of function's body.
/// It means that modifiers must be available in the scope of the marked method.
///
/// Modifiers are designed to be used for methods in impl sections.
/// The method can have several modifiers. They will be expanded from left to right.
/// The modifier can accept arguments from the scope of the method definition
/// (you can pass an argument from the signature of marked method or from the outside scope of function).
/// The modifier accepts arguments only by value and the type of argument must support `Clone` trait,
/// because macro will clone the argument and will pass it to the modifier.
///
/// # Explanation:
///
/// Let's define next modifiers.
/// ```
/// #[brush::modifier_definition]
/// fn A<T>(instance: &T, body: impl Fn(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn B<T, F: Fn(&T) -> &'static str>(instance: &T, body: F, data: u8) -> &'static str {
///     println!("B before {}", data);
///     let result = body(instance);
///     println!("B after {}", data);
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: Fn(&T) -> &'static str
/// {
///     println!("C before");
///     let result = body(instance);
///     println!("C after");
///     result
/// }
///
/// struct Contract {}
///
/// impl Contract {
///     #[brush::modifiers(A, B(_data), C)]
///     fn main_logic(&self, _data: u8) -> &'static str {
///         return "Return value"
///     }
/// }
/// ```
/// The code above will be expanded into:
/// ```
/// #[brush::modifier_definition]
/// fn A<T>(instance: &T, body: impl Fn(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn B<T, F: Fn(&T) -> &'static str>(instance: &T, body: F, data: u8) -> &'static str {
///     println!("B before {}", data);
///     let result = body(instance);
///     println!("B after {}", data);
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: Fn(&T) -> &'static str
/// {
///     println!("C before");
///     let result = body(instance);
///     println!("C after");
///     result
/// }
///
/// struct Contract {}
///
/// impl Contract {
///     fn main_logic(&self, _data: u8) -> &'static str {
///         let mut __brush_body_2 = |__brush_instance_modifier: &Self| {
///             let __brush_cloned_0 = _data.clone();
///             let mut __brush_body_1 = |__brush_instance_modifier: &Self| {
///                 let mut __brush_body_0 = |__brush_instance_modifier: &Self| return "Return value";
///                 C(__brush_instance_modifier, __brush_body_0)
///             };
///             B(__brush_instance_modifier, __brush_body_1, __brush_cloned_0)
///         };
///         A(self, __brush_body_2)
///     }
/// }
///
/// ```
///
/// # Example: Usage
///
/// ```
/// #[brush::contract]
/// mod example {
///     #[ink(storage)]
///     #[derive(Default)]
///     pub struct Contract {
///         initialized: bool,
///         owner: AccountId,
///     }
///
///     #[brush::modifier_definition]
///     fn once(instance: &mut Contract, body: impl Fn(&mut Contract)) {
///         assert!(!instance.initialized, "Contract is already initialized");
///         body(instance);
///         instance.initialized = true;
///     }
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn new() -> Self {
///             Self::default()
///         }
///
///         #[ink(message)]
///         #[brush::modifiers(once)]
///         pub fn init(&mut self, owner: AccountId) {
///             self.owner = owner;
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn modifiers(_attrs: TokenStream, method: TokenStream) -> TokenStream {
    modifiers::generate(_attrs, method)
}
