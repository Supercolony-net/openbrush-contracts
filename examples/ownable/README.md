## Ownable contract

Contract module which provides a basic access control mechanism, where
there is an account (an owner) that can be granted exclusive access to
specific functions.

This module is used through the embedding of `ownable::Data` and implementation of `Ownable` and
`Storage` traits. It will make the modifier `only_owner` available, which can be applied
to your functions to restrict their use to the owner.

[See example](https://supercolony-net.github.io/openbrush-contracts/smart-contracts/ownable)
