# OpenBrush-Contracts

## Summary
**A Rust library for secure smart contract development on ink!.**

This library contains traits definition of standard tokens. 
Everyone who implements these traits will have the same API during cross contract calls.
Also the library provides default implementation on Rust level
([issue](https://github.com/Supercolony-net/openbrush-contracts/issues/5) describes why) 
which can be reused or customized by everyone.

To understand how to use the library better to check [examples](examples) and [doc](doc) folders.

At the moment library is using own version of ink!, but it is the same as [ink!-3.0-rc3](https://github.com/paritytech/ink/releases/tag/v3.0.0-rc3)
with additional [fix](https://github.com/Supercolony-net/ink/commit/4ade565ca0adf746c130ef32e50f54a9504970cb). 
!ink will fix that issue in next release.

## Events feature is disabled
Due to the ink's current state of events generation event feature is currently disabled
As soon as we or ink has fixed this issue event support will be easiy enabled.
[Here](https://github.com/Supercolony-net/openbrush-contracts/issues/2) is a more detailed overview of a problem

## The library is not production-ready, these issues must be resolved first:
* [Standard token naming convention](https://github.com/Supercolony-net/openbrush-contracts/issues/1)
* [Event's identifiers are based on the naming of the storage structure](https://github.com/Supercolony-net/openbrush-contracts/issues/2)

Usage of the library looks not pretty, but it will be simplified with resolving issues:
* [Returning of error doesn't mean revert of transaction](https://github.com/Supercolony-net/openbrush-contracts/issues/3)
* [#[ink::trait_definition] doesn't support generics and default implementation](https://github.com/Supercolony-net/openbrush-contracts/issues/4)
* [Library provides implementation on Rust level instead of ink! level](https://github.com/Supercolony-net/openbrush-contracts/issues/5)
* [List of issues, solving each of them can simplify usage of library](https://github.com/Supercolony-net/openbrush-contracts/issues/8)

The upgradable contract will be available after resolving of this [issue](https://github.com/Supercolony-net/openbrush-contracts/issues/7)

## FAQ

### Why IntelliJ rust plugin doesn't autocomplete the library's derive macros?
It requires enabling of several experimental features:
* `org.rust.cargo.evaluate.build.scripts` - enables building and collecting build artefacts including proc-macro libraries during importing of project structure
* `org.rust.macros.proc` - enables expansion of procedural macros

To enable an experimental feature, type `Experimental feature` in the dialog of `Help | Find Action` action and enabled the corresponding item.

### Was it audited?
Contracts in this repository have not yet been audited. But it is in plans.

## License

OpenBrush is released under the [MIT License](LICENSE).
