---
sidebar_position: 2
title: Setup the project
---

In the first step, we will define the structure of the project.
We suggest using that structure during development for the following reasons:
- The interface of the contracts is defined separately from the contracts. That allows others to communicate with these contracts without knowledge about the implementation and these interfaces can easily be imported to another project(that allows others to communicate with these contracts).
- Resolves the problem with cyclic dependencies across the project. To call the methods of the contract from the project you enough to have an interface.
- The usage of the `ink-as-dependency` feature is minimized. That can resolve a lot of headaches in the future.
- The implementation of big contracts can be split into small parts to simplify the development.
- The body of the contract doesn't contain the whole implementation of the contract. That improves the readability of the contracts.

The project will contain the following directories:
- `traits` - contains all traits(interfaces) of the contracts developed in the project.
Traits describe the functionality of each contract and allow to do cross-contracts calls
without knowledge about the implementation(no need to import the contract, using a trait is enough).
- `impls` - contains the implementations of traits for the contracts. 
If the contract contains several simple functions, better to implement 
them in the body of the contract. But if the contract contains a lot of logic
and methods, better to move(and maybe split on parts) the implementation to that directory.
Better to store the implementation of one contract in its own directory and not mix it with others.
- `derive` - is optional directory. OpenBrush provides the [macro](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/src/derive.rs) 
to define a procedure derive macro of the storage trait. 
That derive can be used to implement the storage trait for the data structure 
in two lines(more details about it later). 
If the developer prefers to use derive macro for his data structures 
then he can define them in that directory to import later in the project
(procedure macros in Rust requires a separate crate).
- `contracts` - contains the bodies of the contracts. Each contract should be defined 
in its own crate(it is a rule of the ink!). Each folder in that directory is a 
crate(contract). These contracts can have the implementation inside themselves 
or they can import the implementation from `impls`.

In that structure `traits`, `impls`, and `derive` directories are the parts of on `PROJECT_NAME` crate.
Each contract in the `contracts` directory imports the crate `PROJECT_NAME` and use it inside.

Based on the rules above the structure will look like the following:
```shell
├── traits
│   ├── lending.rs
│   ├── loan.rs
│   ├── mod.rs
│   ├── shares.rs
│   └── stable_coin.rs
├── impls
│   ├── lending
│   │   ├── data.rs
│   │   ├── lending.rs
│   │   ├── lending_permissioned.rs
│   │   └── mod.rs
│   └── mod.rs
├── derive
│   ├── Cargo.toml
│   └── lib.rs
├── contracts
│   ├── lending
│   │   ├── Cargo.toml
│   │   └── lib.rs
│   ├── loan
│   │   ├── Cargo.toml
│   │   └── lib.rs
│   ├── shares
│   │   ├── Cargo.toml
│   │   └── lib.rs
│   └── stable_coin
│       ├── Cargo.toml
│       └── lib.rs
├── lib.rs
├── Cargo.toml
```

`traits` directory contains 4 traits with logic from the [overview](/smart-contracts/example/overview).
In the example:
- `LendingContract` is a big contract and we moved the main logic into `impls/lending` folder. That logic is split into two traits(`Lending` and `LendingPermissione`) to show how it can be done. Also, the implementation requires the usage of `derive`.
- `LoanContract` contains few methods, so the implementation is defined directly in the body of the contract.
- `SharesContract` is `PSP22` token with public `mint` and `burn` functions that require ownership.
- `StableCoinContract` is a pure `PSP22` token.
