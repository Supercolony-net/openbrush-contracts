## PSP22 Pallet contract (ERC20 analogue)

Implementation of [PSP22](https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md) token standard via [Pallet assets](https://github.com/727-Ventures/pallet-assets-chain-extension) chain extension in Polkadot blockchain.

This will allow for communication between smart-contract based `PSP22` fungible tokens, and `pallet-asset` based fungible tokens. We can instantiate a new `psp22_pallet` contract, which will also instantiate a new asset in the `pallet-asset`. After that, we can use this instantiated smart contract to manipulate with the newly created token, as well as manipulating with the asset as with any asset on the `pallet-asset`.

With this we can achieve usage of `pallet-asset` based tokens within ink! smart contracts, which are based on the `PSP22` standard (as the `PSP22Pallet` implements the `PSP22` standard functions), and we can even teleport our smart-contract based token to chains which do not support smart contracts but do support `pallet-asset` (such as Statemint). You can read more about the asset teleportation [here](https://medium.com/@krikolkk/xcm-and-cross-chain-asset-transferring-6922a0ba209).

[See example](https://727-ventures.github.io/openbrush-contracts/smart-contracts/PSP22-Pallet/)
