# OpenBrush-Contracts
**A Rust library for secure smart contract development on ink!.**

This library contains traits definition of standard tokens. 
Everyone who implements these traits will have the same API during cross contract calls.
Also the library provides default implementation on Rust level
([issue](https://github.com/Supercolony-net/openbrush-contracts/issues/5) describes why) 
which can be reused or customized by everyone.

To understand how to use the library better to check [examples](examples) and [doc](doc) folders.

At the moment library is using own version of ink!, but it is the same [ink!-3.0-rc3](https://github.com/paritytech/ink/releases/tag/v3.0.0-rc3)
with additional [fix](https://github.com/Supercolony-net/ink/commit/4ade565ca0adf746c130ef32e50f54a9504970cb). 
!ink will fix that issue in next release.

The library is not production-ready, these issues must be resolved first:
* https://github.com/Supercolony-net/openbrush-contracts/issues/1
* https://github.com/Supercolony-net/openbrush-contracts/issues/2

Usage of the library looks not pretty, but it will be simplified with resolving issues:
* https://github.com/Supercolony-net/openbrush-contracts/issues/3
* https://github.com/Supercolony-net/openbrush-contracts/issues/4
* https://github.com/Supercolony-net/openbrush-contracts/issues/5
* https://github.com/Supercolony-net/openbrush-contracts/issues/6
* https://github.com/Supercolony-net/openbrush-contracts/issues/8

The upgradable contract will be available after resolving of https://github.com/Supercolony-net/openbrush-contracts/issues/7

## Was it audited?

Contracts in this repository have not yet been audited. But it is in plans.

## License

OpenBrush is released under the [MIT License](LICENSE).