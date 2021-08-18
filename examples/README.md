## Overview

This folder contains example of how the library can be use & how to customize the base implementation.

* [PSP22](psp22) shows an example of how you can reuse the implementation of
  [psp22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token(by the same way you can reuse
  [psp721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) and [psp1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155)).
* [Access Control](access-control) shows how you can use the implementation of
  [access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/access-control) and
  [psp721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) together to provide rights to mint and burn NFT tokens.
* [Ownable](ownable) shows how you can use the implementation of
  [ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable) and
  [psp1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155) together to provide rights to mint and burn tokens.
* [ReentrancyGuard](reentrancy-guard) shows how you can use the implementation of
  [non_reentrant](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/reentrancy-guard)
  modifier to prevent reentrancy during certain functions.
* [TimelockController](timelock-controller) shows how you can use the implementation of
  [timelock-controller](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/governance/timelock-controller)
  to execute some transaction with some delay via governance.
* [PaymentSplitter](payment-splitter) shows how you can use the implementation of
  [payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment-splitter)
  to split received native tokens between participants of the contract.