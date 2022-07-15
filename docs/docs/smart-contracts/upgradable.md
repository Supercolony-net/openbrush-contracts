---
sidebar_position: 4
title: Upgradable contract
---
### Upgradable contract

Smart contracts are immutable. But software quality depends on the ability to upgrade  source code in order to produce iterative releases. Certain degree of mutability is needed for bug fixing and potential product improvements.

Upgradeability allows for experimenting and deploying the product at the early stage, always leaving the chance to fix vulnerabilities and progressively add features. Upgradeable contracts are not a Bug if they are developed consciously with decentralization in mind.

There 2 types of Upgradable contract OpenBrush supports

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

Proxy upgradable contract contains state variable `forward_to` in ink! storage that store Hash to uploaded code. Upgradable contract contains `change_delegate_call` method to update Hash for `forward_to` value inside the contract. Only owner is able to call `change_delegate_call` method.

Upgradable contracts using proxy:

* Executes any call that does not match a selector of itself with the code of another contract.
* The other contract does not need to be deployed on-chain.
* State is stored in the storage of the originally called contract.

This is the illustration how Proxy contract with delegate_call looks like:

![](assets/20220715_130416_DD058578-67E2-4832-9F75-CA18C3B3921C_4_5005_c.jpeg)

Upgradable contract store `forward_to` Hash inside storage by `STORAGE_KEY`

```rust

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct Data {
    pub forward_to: Hash,
}

```

OpenBrush implement default `Proxy` upgradable functionality for any type that implement `Storage<Data>` and `Storage<ownable::Data>>` traits

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

### With OpenBrush it is too easy create your own proxy upgradable contract

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

Diamond upgradable contract stores:

- selector mapped to its facet
- facet mapped to all functions it supports

When you create a new contract (facet), which you want to make delegate calls from your diamond contract to, you will call the `diamond_cut` function on your diamond contract, with the code hash of your new facet and the selectors of all the functions from this facet you want to use. The diamond will register them and anytime you call this function on your diamond contract, it will make the delegate call to the facet the function belongs to. You can add, remove or replace these functions anytime with the `diamond_cut` function, some of the limitations are, that you can not add functions with the same selectors, when replacing functions, the new function needs to be from a different contract, then currently in use, and when removing functions, the function needs to be registered in the diamond contract.
