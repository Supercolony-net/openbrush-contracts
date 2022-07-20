## Raw Diamond vs ink! codegen Standard

The folder contains ink! version of a simple diamond standard with minimized functionality to reduce the size of the contract and folder with the exact implementation but without codegen of ink!. This version still uses primitives from ink! to make code more readable.

The size already is less, but it is an excellent playground to better understand the influence of each part of the code. For example, the usage of `CallBuilder` for delegate calls is much more expensive than the usage of `seal_delegate_call`.

Usage of `Mapping` also adds overhead, and the implementation with raw `seal_get_storage` and `seal_set_storage` is better.

Using the same business code in `deploy` and in `call` functions, the size is still less due to codegen overhead.

It is only an example for future work with optimizations.