# openbrush-contracts
**A Rust library for secure smart contract development on ink!.**

* Implementations of standards like [ERC20](https://docs.openzeppelin.com/contracts/erc20) and [ERC721](https://docs.openzeppelin.com/contracts/erc721).
  (TODO: [PSP](https://github.com/w3f/PSPs) must be defined first for right naming convention, 
  because ERC it is naming from Ethereum, we need to have own naming)
* Flexible [role-based permissioning](https://docs.openzeppelin.com/contracts/access-control) scheme.
* Anyone can reuse/extend/customize the implementation. You can check `examples` folder for better understanding.

At the moment library is using own version of ink!, but it is the same [ink!-3.0-rc3](https://github.com/paritytech/ink/releases/tag/v3.0.0-rc3)
with additional [fix](https://github.com/Supercolony-net/ink/commit/4ade565ca0adf746c130ef32e50f54a9504970cb). !ink will fix that issue in next release.

It is alpha version of the library and it is not the final variant.
It will be updated according to changes/fixes in ink! and it also will be supplemented by another contracts.

ink! has issues which doesn't allow to EASY reuse the code at the moment. But it will resolved in future so the library will be more comfortable for usage.
You can read an article about [issues](https://medium.com/supercolony/ink-has-most-of-the-features-required-for-usage-however-the-usability-of-ink-is-low-95f4bc974e22) 
which must be first resolved.

TODO: Add the list of issues - blockers