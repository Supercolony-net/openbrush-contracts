## Overview

[`traits`](src/traits) contains definitions of traits(interfaces) for the popular contracts.
Anyone can import the trait and use it to do cross-contract calls without
knowing the implementation of the contract.

This folder contains the implementation of popular contracts.
Anyone can re-use the implementation after importing the `openbrush` crate, enabling
the desired feature(s) of the contract, and implementing all required traits.
