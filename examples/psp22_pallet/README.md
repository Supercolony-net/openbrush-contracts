## PSP22 Pallet contract (ERC20 analogue)

Implementation of [PSP22](https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md) token standard via [Pallet assets](https://github.com/727-Ventures/pallet-assets-chain-extension) chain extension in Polkadot blockchain.

This implementation standardizes the `pallet-assets` assets into `PSP22` standard. So the functions within this implementation resemble the `PSP22` standard, while those functions are calling the underlying functions of `pallet-asset`. Here is how it works:

- the constructor will create a new asset on the `pallet-assets`
- we can now manipulate with this asset as with any other asset on `pallet-assets`
- however we can also call the functions of this smart contract, and it will call the underlying functions from `pallet-assets`

With this we can achieve usage of `pallet-assets` based tokens within ink! smart contracts, which are based on the `PSP22` standard (as the `PSP22Pallet` implements the `PSP22` standard functions), and we can even transfer our smart-contract based token to chains which do not support smart contracts but do support `pallet-assets` (such as Statemint). You can read more about the cross-chain asset transfer and see a guide [here](https://medium.com/@krikolkk/xcm-and-cross-chain-asset-transferring-6922a0ba209).

[See example](https://727-ventures.github.io/openbrush-contracts/smart-contracts/PSP22-Pallet/)
