sidebar_position: 4
title: Upgradeable contract
--------------------------

# Upgradeable contract

## Overview

Smart contracts are immutable by default, adding a layer of security and trust to the 
contracts. But software quality depends on the ability to upgrade source code to 
produce iterative releases. A certain degree of mutability is needed for bug fixing 
and potential product improvements.

Upgradeability allows for experimenting and deploying the product at the early stage, 
always leaving the chance to fix vulnerabilities and progressively add features. 
It is more actual right now while ink! and contract-pallet under active development. 
Upgradeable contracts are not a Bug if they are developed consciously with 
decentralization in mind.

Decentralization can be achieved by providing the right to upgrade only to 
decentralized authority like governance, multisig, or another analog.

It is not hard to upgrade the logic of contracts. It can be achieved via `Proxy` and `Diamond` 
patterns or via the `set_code_hash` function provided by contract-pallet. The hardest 
part is to save the contract's state and make it compatible with new logic.

## Storage layout

### How storage works

Contracts use key-value storage to persist data. Each field of the contract may have 
its key and occupy storage cell. It is called storage layout.

![](assets/20220719_075309_F016550A-65D2-4DCD-A60A-D1A70B38D813.jpeg)

During compilation ink! inserts code to work with storage and ink! knows how to store 
each type in which storage cell. How exactly it works is not a part of that tutorial. 
The main point is that each type knows how to operate with each field and operate with 
storage, cause of a unique identifier. In the old version of ink! the identifier is 
`[u8; 32]` in a [new version](https://github.com/paritytech/ink/issues/1134) it is `u32`.

So, each data is stored under its unique identifier - the storage key. The value of the 
key is the sequence of bytes - serialized(by SCALE codec) data type. The logic layer 
knows how to serialize and deserialize each data type. So during the execution, the 
logic layer deserializes all data by their storage keys and returns the 
filled contract's storage variable. The developer works with that variable, and before 
the end of the execution, the logic layer serializes data into sequences of bytes and 
stores them in their storage cells.

### Rules for upgradeable storage layout

The contract can have several logic layers (like in the `Diamond` pattern). 
So we will define rules in terms of several layers, but it is also applicable 
for upgradeable contracts with the `Proxy` pattern or `set_code_hash`.

1. The data stored under the storage key should use the same serialization and 
deserialization mechanism across all logic layers. Otherwise, some logic layers will not 
be able to deserialize the data type and fail.
1. Each logic unit(in most cases, it is a type) should occupy the same storage key across
all logic layers. For example, you have several logic layers that use the 
`Mapping<Owner, Balances>` to track users' balances. You should use the same storage 
key if you want to interact with the same mapping. Otherwise, you will work with different mappings.
1. Each field that occupies some storage key should be used only in its usage flow. 
For example, you have `Mapping<Owner, Balances>` to track users' balances of token 
A in one logic layer. You can't use it in another logic layer to track users' 
balances of token B. Otherwise, one logic layer can overwrite another.

Following those simple rules will save you from corrupting the storage. 
Those rules are applicable for upgraded logic layers too.

If you use the same storage layout across all logic layers, and you don't plan to have 
unique fields per layer(so you don't plan to modify the storage layout in future upgrades). 
Then you already follow those rules with automatically calculated storage keys. 
But if you want to use a unique layout per layer or 
plan to modify the layout in the future, the next section will help you.

### Suggestions on how follow the rules

#### Approach description

The manual setting of the storage key for each field allows following the rules but 
makes development harder. ink! allows you to manually implement all utility traits and 
specify the storage key you want to use for each field. If your contract has 20 fields, 
then you need to set 20 storage keys.

The main suggestion is to design your storage layout as a bunch of logic units and 
assign a unique storage key per logic unit. The logic unit can be one field or a bunch 
of fields. In the scope of the logic unit, you can use automatically calculated keys 
offsetted with the storage key of the logic unit, or you can use the same approach 
again and split logic into more units.

With that approach, you can order your units as you wish. You can add/remove/swap 
them and don't worry about storage layout because each logic unit will have its space 
in the blockchain's storage. If storage keys are unique, those spaces don't overlap.

OpenBrush provides [`openbrush:: upgradeable_storage`](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs#L447) 
attribute macro that implements all required traits with specified storage key(storage key is required input argument to macro). 
Also, macro initializes the field with a default value if the field is not initialized before
(it can be actual during the upgrade because new fields are not initialized yet).
You can use that macro to define a logic unit.

#### Logic unit per business use case

You can include all fields into logic unit, like this:
```rust
#[openbrush::upgradeable_storage(0x123)]
pub struct Data {
    balances: Mapping<Owner, Balance>,
    total_owners: u128,
}
```

It makes your code readable and segregated by business logic. 
But it will add some limitations to future upgrades.

##### Limitations for future upgrades

Each field that doesn't have a separate space in the storage almost always depends 
on the field ordering(and maybe naming if you use a new ink!). So you can't remove 
fields or change the ordering(and naming). 

But you can add new fields. For that, you can reserve one field with empty type `Option<()>` 
in your contract for future type.

```rust
#[openbrush::upgradeable_storage(0x123)]
pub struct Data {
    balances: Mapping<Owner, Balance>,
    total_owners: u128,
    _reserved: Option<()>,
}
```

The default value of that field is `None`. But in the future, you can init it with some useful type and value.

```rust
#[openbrush::upgradeable_storage(0x123)]
pub struct Data {
    balances: Mapping<Owner, Balance>,
    total_owners: u128,
    _reserved: Option<DataExtend>,
}

impl Data {
    fn extension(&mut self) -> &mut DataExtension {
        &mut self._reserved.unwrap_or_default()
    }
}

#[derive(Default)]
pub struct DataExtension {
  owners_blacklist: Mapping<Owner, ()>,
  _reserved: Option<()>,
}
```

So if you modify your contract many times in the future,
it can cause a deep stack of `_reserved` fields, or many dead fields.
You can always create a new logic unit and embed the old one. So you should decide what 
is better for you right now. Create a new logic unit that will include the old one, 
or add a new field into the current.

#### Logic unit per each field

You can create a unique type for each field like:

```rust
#[openbrush::upgradeable_storage(0x123)]
pub struct Balances(openbrush::storage::Mapping<AccountId, Balance>);

#[openbrush::upgradeable_storage(0x124)]
pub struct TotalOwners(u128);
```

You have no limitations, but you made your code harder to read, 
and maybe you have a lot of unique structures :D

#### Unique storage key

The storage key should be unique per each logic unit. You can assign each key manually or 
use some hash function to automate it.

OpenBrush provides [`openbrush::storage_unique_key!`](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/src/macros.rs#L25) 
macro that generates a storage key based on the path to the structure. 
It has one required input argument - the name of the structure.

```rust
#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
pub struct Data {
    balances: Mapping<Owner, Balance>,
    total_owners: u128,
    _reserved: Option<()>,
}
```

or 

```rust
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    balances: Mapping<Owner, Balance>,
    total_owners: u128,
    _reserved: Option<()>,
}
```

## Types of upgradeable contracts

There 2 types of Upgradeable contract OpenBrush supports

- **Proxy**

  - Pros
    - Basic patern where hard to introduce a bug
  - Cons
    - Necessity to deploy extra contract and additional overhead for every singe call
- **Diamond standart**

  - Pros
    - Allows to split logic on facets to save execution fees and overcome contract size limits
    - Allows to upgrade facets separately and use different governance rules
  - Cons
    - More overhead for particular facets
    - More likely to brick the storage

### Upgrading via the Proxy Pattern

The basic idea is using a proxy for upgrades. The first contract is a simple wrapper or "proxy" which users interact with directly and is in charge of forwarding transactions to and from the second contract, which contains the logic. The logic contract can be replaced while the proxy, or the access point is never changed. Both contracts are still immutable in the sense that their code cannot be changed, but the logic contract can simply be swapped by another contract.

Proxy upgradeable contract contains state variable `forward_to` that store Hash to uploaded code. Upgradeable contract contains `change_delegate_call` method to update Hash for `forward_to` value inside the contract. Only owner is able to call `change_delegate_call` method.

Upgradeable contracts using proxy:

* Executes any call that does not match a selector of itself with the code of another contract.
* The other contract does not need to be deployed on-chain.
* State is stored in the storage of the originally called contract.

This is the illustration how Proxy contract with delegate_call looks like:

![](assets/20220715_130416_DD058578-67E2-4832-9F75-CA18C3B3921C_4_5005_c.jpeg)

Upgradeable contract store `forward_to` Hash inside storage by `STORAGE_KEY`

```rust

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct Data {
    pub forward_to: Hash,
}

```

OpenBrush implement default `Proxy` upgradeable functionality for any type that implement `Storage<Data>` and `Storage<ownable::Data>>` traits

```rust
impl<T: Storage<Data> + Storage<ownable::Data>> Proxy for T {
    default fn get_delegate_code(&self) -> Hash {
        self.data::<Data>().forward_to
    }

    #[modifiers(ownable::only_owner)]
    default fn change_delegate_code(&mut self, new_code_hash: Hash) -> Result<(), OwnableError> {
        let old_code_hash = self.data::<Data>().forward_to.clone();
        self.data::<Data>().forward_to = new_code_hash;
        self._emit_delegate_code_changed_event(Some(old_code_hash), Some(new_code_hash));
        Ok(())
    }
}
```

Implementation of `Storage<ownable::Data>>` give possiblity to use `only_owner` modifier that allows only owner calls `change_delegate_code` method.

In additional,  OpenBrush implement `Internal` trait for any type that implement `Storage<Data>`
This implementation adds default `fallback` functionality that will DelegateCall to other contract by some code hash.

```rust
impl<T: Storage<Data>> Internal for T {

    default fn _fallback(&self) -> ! {
        ink_env::call::build_call::<ink_env::DefaultEnvironment>()
            .call_type(DelegateCall::new().code_hash(self.data().forward_to.clone()))
            .call_flags(
                ink_env::CallFlags::default()
                // We don't plan to use the input data after the delegated call, so the 
                // input data can be forwarded to delegated contract to reduce the gas usage.
                .set_forward_input(true)
                // We don't plan to return back to that contract after execution, so we 
                // marked delegated call as "tail", to end the execution of the contract.
                .set_tail_call(true),
            )
            .fire()
            .unwrap_or_else(|err| {
                panic!(
                    "delegate call to {:?} failed due to {:?}",
                    self.data().forward_to.clone(),
                    err
                )
            });
        unreachable!("the _fallback call will never return since `tail_call` was set");
    }
}
```

### With OpenBrush it is so easy create your own proxy upgradeable contract

- create your struct and use `Storage` derive macro and use `ownable::Data` + `roxy::Data` storage field

```rust
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyProxy {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        proxy: proxy::Data,
    }
```

- create `constructor` and call `_init_with_forward_to` inside `new` asociated function.

```rust

    impl MyProxy {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            let mut inst = Self::default();
            inst._init_with_forward_to(Hash::try_from(forward_to).unwrap());
            inst._init_with_owner(Self::env().caller());
            inst
        }
    }
```

- Implement `Proxy` and `proxy::Internal` trait for your struct

```rust
    impl Proxy for MyProxy {}

    impl proxy::Internal for MyProxy {}
```

Now you can easily deploy and call your contract and `fallback` method will forward call to other contract using `code_hash`

### Upgrading via the Diamond Standart

Using Diamond Standart you can add support for different facets (contracts) and their functions and remove or replace existing functions from the contract.

This is the illustration how contract with Diamond standart patern looks like:

![](assets/20220715_130335_47FD0F8D-60F3-4FDF-82F4-672402FDC5D1.jpeg)

These things to understand diamonds:

1. A diamond is a smart contract. Its substrate address is the single address that outside software uses to interact with it.
2. Internally a diamond uses a set of contracts called facets for its external functions.
3. All state variable storage data is stored in a diamond, not in its facets.
4. The external functions of facets can directly read and write data stored in a diamond. This makes facets easy to write and gas efficient.
5. A diamond is implemented as a fallback function that uses delegatecall to route external function calls to facets.
6. A diamond often doesn’t have any external functions of its own — it uses facets for external functions which read/write its data.

A diamond is deployed by adding at least a facet to add the ‘diamondCut’ or other upgrade function in the constructor of the diamond. Once deployed more facets can be added using the upgrade function.

OpenBrush library implements Diamond standart with DiamondCut struct and defailt implementation of `diamond_cut` method. Only of contract can call this method to update facets.

Diamond upgradeable contract stores those data:

- selector mapped to its facet
- facet mapped to all functions it supports

```rust
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data<D: DiamondCut = ()> {
    // Selector mapped to its facet
    pub selector_to_hash: Mapping<Selector, Hash>,
    // Facet mapped to all functions it supports
    pub hash_to_selectors: Mapping<Hash, Vec<Selector>>,
    // Handler of each facet add and remove.
    // It is empty by default but can be extended with loup logic.
    pub handler: D,
}
```

`DiamondCut` has `openbrush::upgradeable_storage` macros which implements `SpreadLayout`, `SpreadAllocate`, `StorageLayout` and `OccupyStorage` with a specified storage key instead of the default one (All data is stored under the provided storage key).

When you create a new contract (facet), which you want to make delegate calls from your diamond contract to, you will call the `diamond_cut` function on your diamond contract, with the code hash of your new facet and the selectors of all the functions from this facet you want to use. The diamond will register them and anytime you call this function on your diamond contract, it will make the delegate call to the facet the function belongs to. You can add, remove or replace these functions anytime with the `diamond_cut` function, some of the limitations are, that you can not add functions with the same selectors, when replacing functions, the new function needs to be from a different contract, then currently in use, and when removing functions, the function needs to be registered in the diamond contract.
