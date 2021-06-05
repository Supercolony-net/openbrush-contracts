## Overview
This folder contains example of how the library can be reuse/customize 
the base implementation.
* [Erc20](erc20) shows an example of how you can reuse the implementation of 
  [erc20](contracts/token/erc20) token(by the same way you can reuse 
  [erc721](contracts/token/erc721) and [erc1155](contracts/token/erc1155)).
* [Access Control](access-control) shows how you can use the implementation of
  [access-control](contracts/access/access-control) and
  [erc721](contracts/token/erc721) together to provide rights
  to mint and burn NFT tokens.