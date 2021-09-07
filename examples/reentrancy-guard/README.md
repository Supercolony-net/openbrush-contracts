## ReentrancyGuard modifier

Prevents a contract from calling itself, directly or indirectly.
Calling a `non_reentrant` function from another `non_reentrant`
function is not supported. It is possible to prevent this from happening
by making the `non_reentrant` function external, and make it call a
`private` function that does the actual work.

This modifier flushes the struct into storage with `ENTERED`
status before calling the original method.

[See example](https://supercolony-net.github.io/openbrush-contracts/smart-contracts/reentrancy-guard)