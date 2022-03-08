#![feature(drain_filter)]
extern crate proc_macro;

mod contract;
mod internal;
mod metadata;
mod modifier_definition;
mod modifiers;
mod trait_definition;
mod wrapper;

use proc_macro::TokenStream;

/// Entry point for use brush's macros in ink! smart contracts.
///
/// # Description
///
/// The macro consumes brush's macros to simplify the usage of the library.
/// After consumption, it pastes ink! code and then ink!'s macros will be processed.
///
/// This macro consumes impl section for traits defined with [`#[brush::trait_definition]`](`macro@crate::trait_definition`).
#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    contract::generate(_attrs, ink_module)
}

/// Defines extensible trait in the scope of brush::contract.
/// It is a common rust trait, so you can use any features of rust inside of this trait.
/// If this trait contains some methods marked with `#[ink(message)]` or `#[ink(constructor)]` attributes,
/// this macro will extract these attributes and will put them into a separate trait
/// (the separate trait only is used to call methods from the original trait), but the macro will not touch methods.
///
/// This macro stores definition of the trait in a temporary file during build process.
/// Based on this definition [`#[brush::contract]`](`macro@crate::contract`)
/// will generate implementation of additional traits.
///
///  ** Note ** The name of the trait defined via this macro must be unique for the whole project.
///  ** Note ** You can't use aliases, generics, and other rust's stuff in signatures of ink!'s methods.
///
/// # Example: Definition
///
/// ```
/// mod doc {
/// use ink_prelude::collections::BTreeMap;
/// use brush::traits::{AccountId, Balance, InkStorage};
///
/// #[derive(Default, Debug)]
/// pub struct Data {
///     pub balances: BTreeMap<AccountId, Balance>,
/// }
///
/// #[brush::trait_definition]
/// pub trait PSP22Storage: InkStorage {
///     fn get(&self) -> &Data;
///     fn get_mut(&mut self) -> &mut Data;
/// }
///
/// #[brush::trait_definition]
/// pub trait PSP22: PSP22Storage {
///     /// Returns the account Balance for the specified `owner`.
///     #[ink(message)]
///     fn balance_of(&self, owner: AccountId) -> Balance {
///         self.get().balances.get(&owner).copied().unwrap_or(0)
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
///         self.get_mut().balances.insert(from, from_balance - amount);
///         let to_balance = self.balance_of(to);
///         self.get_mut().balances.insert(to, to_balance + amount);
///     }
/// }
/// }
/// ```
///
/// # Example: Implementation
///
/// It uses storage trait from above.
///
/// ```
/// #[brush::contract]
/// mod base_psp22 {
///     use ink_prelude::collections::BTreeMap;
///     use brush::traits::InkStorage;
///     use ink_storage::traits::StorageLayout;
///     use ink_storage::traits::SpreadLayout;
///
///     #[derive(Default, Debug, SpreadLayout)]
///     #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
///     pub struct Data {
///         pub supply: Balance,
///         pub balances: BTreeMap<AccountId, Balance>,
///         pub allowances: BTreeMap<(AccountId, AccountId), Balance>,
///     }
///
///     #[brush::trait_definition]
///     pub trait PSP22ExampleStorage: InkStorage {
///         fn get(&self) -> &Data;
///         fn get_mut(&mut self) -> &mut Data;
///     }
///
///     #[brush::trait_definition]
///     pub trait PSP22Example: PSP22ExampleStorage {
///         /// Returns the account Balance for the specified `owner`.
///         #[ink(message)]
///         fn balance_of(&self, owner: AccountId) -> Balance {
///             self.get().balances.get(&owner).copied().unwrap_or(0)
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
///             self.get_mut().balances.insert(from, from_balance - amount);
///             let to_balance = self.balance_of(to);
///             self.get_mut().balances.insert(to, to_balance + amount);
///         }
///     }
///
///     #[ink(storage)]
///     #[derive(Default)]
///     pub struct PSP22Struct {
///         example: Data,
///         hated_account: AccountId,
///     }
///
///     impl PSP22ExampleStorage for PSP22Struct {
///         fn get(&self) -> &Data {
///             &self.example
///         }
///
///         fn get_mut(&mut self) -> &mut Data {
///             &mut self.example
///         }
///     }
///
///     impl PSP22Example for PSP22Struct {
///         // Let's override method to reject transactions to bad account
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             assert!(to != self.hated_account, "I hate this account!");
///
///             let from_balance = self.balance_of(from);
///             assert!(from_balance >= amount, "InsufficientBalance");
///             self.get_mut().balances.insert(from, from_balance - amount);
///             let to_balance = self.balance_of(to);
///             self.get_mut().balances.insert(to, to_balance + amount);
///         }
///     }
///
///     impl PSP22Struct {
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

/// This macro only checks that some free-standing function satisfies a set of rules.
///
/// Rules:
/// - First argument should not be `self`.
/// - First argument must be a reference to a type `instance: &T`. In most cases it's the instance of contract.
/// - Second argument is function's body(this function contains the main code of method attached to the modifier).
/// The type must be `Fn(&T)`, `FnMut(&T)` or `FnOnce(&T)`.
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
/// fn once<BodyFn: FnOnce(&mut Contract)>(instance: &mut Contract, body: BodyFn, _example_data: u8) {
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
/// fn A<T>(instance: &T, body: impl FnOnce(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn B<T, F: FnOnce(&T) -> &'static str>(instance: &T, body: F, data: u8) -> &'static str {
///     println!("B before {}", data);
///     let result = body(instance);
///     println!("B after {}", data);
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: FnOnce(&T) -> &'static str
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
///         return "Return value";
///     }
/// }
/// ```
/// The code above will be expanded into:
/// ```
/// #[brush::modifier_definition]
/// fn A<T>(instance: &T, body: impl FnOnce(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn B<T, F: FnOnce(&T) -> &'static str>(instance: &T, body: F, data: u8) -> &'static str {
///     println!("B before {}", data);
///     let result = body(instance);
///     println!("B after {}", data);
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: FnOnce(&T) -> &'static str
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
///                 let mut __brush_body_0 = |__brush_instance_modifier: &Self| return "Return value";;
///                 C(__brush_instance_modifier, __brush_body_0)
///             };
///             B(__brush_instance_modifier, __brush_body_1, __brush_cloned_0)
///         };
///         A(self, __brush_body_2)
///     }
/// }
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
///     fn once(instance: &mut Contract, body: impl FnOnce(&mut Contract)) {
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

/// This macro allows you to define a wrapper type for traits defined via
/// [`#[brush::trait_definition]`](`macro@crate::trait_definition`).
/// It is a wrapper for `AccountId` that knows how to do cross-contract calls to another contract.
///
/// To define a wrapper you need to use the follow construction:
/// `type TraitName = dyn Trait_1 + Trait_2 ... + Trait_n`, where `Trait_i` contains ink! messages
/// and defined via [`#[brush::trait_definition]`](`macro@crate::trait_definition`).
/// If `Trait_i` doesn't contain ink! messages, then you don't need to create a wrapper for that trait,
/// because the wrapped methods are created only for ink! messages. Otherwise, you will get an error like
///
/// `use of undeclared crate or module `trait_i_external``
///
///  ** Note ** The first argument of method should be a reference on `AccountId` of callee
/// contract(even if the signature of the method requires a mutable reference).
///  ** Note ** Crated wrapper is only a type, so you can't create an instance of this object.
///  ** Note ** The wrapper contains only ink's methods of the trait, it doesn't include a method of super traits.
/// If you want to wrap them too, you need to explicitly specify them.
///
/// # Example: Definition
///
/// ```should_panic
/// {
/// use brush::traits::AccountId;
///
/// #[brush::trait_definition]
/// pub trait Trait1 {
///     #[ink(message)]
///     fn foo(&mut self) -> bool;
/// }
///
/// #[brush::wrapper]
/// type Trait1Ref = dyn Trait1;
///
/// #[brush::trait_definition]
/// pub trait Trait2 {
///     #[ink(message)]
///     fn bar(&mut self, callee: brush::traits::AccountId) {
///         let foo_bool = Trait1Ref::foo(&callee);
///     }
/// }
///
/// #[brush::wrapper]
/// type Trait1and2Ref = dyn Trait1 + Trait2;
///
/// // Example of explicit call
/// let to: AccountId = [0; 32].into();
/// let callee: AccountId = [0; 32].into();
/// Trait1and2Ref::bar(&to, callee);
///
/// // Example of implicit call
/// let to: &Trait1and2Ref = &to;
/// to.bar(callee);
///
/// // Example how to get ink! call builder
/// let to: AccountId = [0; 32].into();
/// let builder_for_foo: ::ink_env::call::CallBuilder<_, _, _, _> = Trait1and2Ref::foo_builder(&to);
/// let ink_result: Result<bool, ink_env::Error> = builder_for_foo.fire();
/// }
/// ```
#[proc_macro_attribute]
pub fn wrapper(attrs: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::generate(attrs, input)
}
