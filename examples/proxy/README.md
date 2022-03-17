## Proxy contract

Contract module which provides am implementation of Proxy pattern for upgradeable contracts.

This module is used through the embedding of `ProxyData` and implementation of `Proxy` and
`ProxyStorage` traits. It will allow us to update contract implementation via a Proxy pattern.
We can get the current contract's implementation code hash or set the new implementation's code hash.
To set a new code hash signer must be the owner of the Proxy.

The example consists of `proxy`, `psp22_upgradeable` and `psp22_metadata_upgradeable` contracts. The goal is to
deploy `proxy` and `psp22_upgradeable` contracts, check that delegate calls through `proxy` contract to `psp22_upgradeable`
work fine and then update contract to `psp22_metadata_upgradeable`.