## Diamond contract

Contract module which provides am implementation of Diamond Standard pattern for upgradeable contracts.

This module is used through the embedding of `DiamondData` and implementation of `Diamond` and
`DiamondStorage` traits. It will allow us to update contract implementation via the Diamond Standard pattern.
We can add support for different facets (contracts) and their functions and remove or replace existing functions
from the contract.

The example consists of `diamond`, which is the main contract in the standard and will be used for storage and 
execution of it's modules' functions via fallback, and the `psp22_facet`, which is the facet with functionality of basic 
[PSP-22](https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md) token.