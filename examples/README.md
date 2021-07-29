## Overview
This folder contains example of how the library can be reuse/customize 
the base implementation.
* [PSP22](psp20) shows an example of how you can reuse the implementation of 
  [psp20](contracts/token/psp20) token(by the same way you can reuse 
  [psp721](contracts/token/psp721) and [psp1155](contracts/token/psp1155)).
* [Access Control](access-control) shows how you can use the implementation of
  [access-control](contracts/access/access-control) and
  [psp721](contracts/token/psp721) together to provide rights
  to mint and burn NFT tokens.
* [Ownable](ownable) shows how you can use the implementation of
  [ownable](contracts/access/ownable) and
  [psp1155](contracts/token/psp1155) together to provide rights
  to mint and burn tokens.
* [ReentrancyGuard](reentrancy_guard) shows how you can use the implementation of
  [non_reentrant](contracts/security/reentrancy_guard) 
  modifier to prevent reentrancy during certain functions.