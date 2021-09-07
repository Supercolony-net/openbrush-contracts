## Pausable contract

Contract module, which allows children to implement an emergency stop
mechanism that an authorized account can trigger.

This module is used through the embedding of `PausableData` and implementation of `Pausable` and
`PausableStorage` traits. It will make available the modifier `when_not_paused` and `when_paused`,
which can be applied to your functions to restrict their usage.

[See example](https://supercolony-net.github.io/openbrush-contracts/smart-contracts/pausable)

