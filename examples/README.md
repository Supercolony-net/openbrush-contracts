## Overview

This folder contains examples of how the library can be used & how to customize the base implementation.

* [PSP22](psp22) shows an example of how you can reuse the implementation of
  [psp22](../contracts/token/psp22) token (in the same way you can reuse
  [psp721](../contracts/token/psp721) and [psp1155](../contracts/token/psp1155)).
* [Access Control](access-control) shows how you can use the implementation of
  [access-control](../contracts/access/access-control) and
  [psp721](../contracts/token/psp721) together to provide rights to mint and burn NFT tokens.
* [Ownable](ownable) shows how you can use the implementation of
  [ownable](../contracts/access/ownable) and
  [psp1155](../contracts/token/psp1155) together to provide rights to mint and burn tokens.
* [ReentrancyGuard](reentrancy-guard) shows how you can use the implementation of
  [non_reentrant](../contracts/security/reentrancy-guard)
  modifier to prevent reentrancy during certain functions.
* [Pausable](pausable) shows how you can use the implementation of
  [pausable](../contracts/security/pausable)
  contract and modifiers.
* [TimelockController](timelock-controller) shows how you can use the implementation of
  [timelock-controller](../contracts/governance/timelock-controller)
  to execute a transaction with some delay via governance.
* [PaymentSplitter](payment-splitter) shows how you can use the implementation of
  [payment-splitter](../contracts/finance/payment-splitter)
  to split received native tokens between participants of the contract.
