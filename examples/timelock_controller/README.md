## TimelockController contract

Contract module which acts as a time-locked controller. When set as the
owner of an `Ownable` smart contract, it enforces a timelock on all
`onlyOwner` maintenance operations. This gives time for users of the
controlled contract to exit before a potentially dangerous maintenance
operation is applied.

By default, this contract is self-administered, meaning administration tasks
have to go through the timelock process. The proposer (resp executor) role
is in charge of proposing (resp executing) operations. A common use case is
to position this `TimelockController` as the owner of a smart contract, with
a multisig or a DAO as the sole proposer.

This module is used through embedding of `TimelockControllerData`, `AccessControlData` and
implementation of `TimelockController`, `TimelockControllerStorage`, `AccessControl` and
`AccessControlStorage` traits.

[See example](https://supercolony-net.github.io/openbrush-contracts/smart-contracts/timelock-controller)
