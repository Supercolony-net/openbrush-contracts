## PaymentSplitter contract

This contract allows splitting native token payments among a group of accounts. The sender does not need to be aware
that the native token will be split in this way, since it is handled transparently by the contract.

The split can be in equal parts or in any other arbitrary proportion. The way this is specified is by assigning each
account to a number of shares. Of all the native tokens that this contract receives, each account will then be able to claim
an amount proportional to the percentage of total shares they were assigned.

`PaymentSplitter` follows a _pull payment_ model. This means that payments are not automatically forwarded to the
accounts but kept in this contract, and the actual transfer is triggered as a separate step by calling the `release`
function. `release` pays out to only the provided address. If you will have many people to pay out, especially if often, you will likely
want to use the `releaseAll` method instead to save you a lot of time.

** Note **: In the substrate balance of contract decreases each block. Because it pays rent for the storage.
So during `release`, each next user will get fewer native tokens.

This module is used through embedding of `PaymentSplitterData` and implementation of `PaymentSplitter` and
`PaymentSplitterStorage` traits.

[See example](https://supercolony-net.github.io/openbrush-contracts/smart-contracts/payment-splitter)
