## Diamond contract

Contract module which provides am implementation of Diamond Standard pattern for upgradeable contracts.

This module is used through the embedding of `DiamondData` and implementation of `Diamond` and
`DiamondStorage` traits. It will allow us to update contract implementation via the Diamond Standard pattern.
We can add support for different facets (contracts) and their functions and remove or replace existing functions
from the contract.

The example consists of `diamond` contract.