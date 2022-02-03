---
sidebar_position: 9
title: Lending contract
---

The main logic of the `LendingContract` is defined in the `impls/lending` directory.
In this file, we only need to "inherit" it.

## Add dependencies

`LendingContract` instantiates the `SharesContract` and `LoanContract`, so we
should import them as `ink-as-dependency`. Also we want to use the `AccessControl`
and `Pausable` from OpenBrush, so we import them too. We also want to "inherit" the
implementation of `Lending` and `LendingPermissioned` traits defined in the `lending_project` crate.

```toml
[package]
name = "lending_contract"
version = "1.0.0"
authors = ["Supercolony <dominik.krizo@supercolony.net>"]
edition = "2018"

[dependencies]
ink_primitives = { tag = "v3.0.0-rc8", git = "https://github.com/paritytech/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc8", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc8", git = "https://github.com/paritytech/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc8", git = "https://github.com/paritytech/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc8", git = "https://github.com/paritytech/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc8", git = "https://github.com/paritytech/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }
scale-info = { version = "1", default-features = false, features = ["derive"], optional = true }

# These dependencies
shares_contract = { path = "../shares", default-features = false, features = ["ink-as-dependency"]  }
loan_contract = { path = "../loan", default-features = false, features = ["ink-as-dependency"]  }
lending_project = { path = "../..", default-features = false }
brush = { path = "../../..", default-features = false, features = ["psp22", "psp34", "pausable", "access_control"] }

[lib]
name = "lending_contract"
path = "lib.rs"
crate-type = [
    "cdylib",
]

[features]
default = ["std"]
std = [
    "ink_primitives/std",
    "ink_metadata",
    "ink_metadata/std",
    "ink_env/std",
    "ink_storage/std",
    "ink_lang/std",
    "scale/std",
    "scale-info",
    "scale-info/std",

    # These dependencies
    "loan_contract/std",
    "shares_contract/std",
    "brush/std",
]
ink-as-dependency = []

[profile.dev]
overflow-checks = false
codegen-units = 16

[profile.release]
overflow-checks = false
```

## Define the contract storage

As described earlier, we want our smart contract to be paused by the Manager account. 
To do that, we need our contract to be `Pausable` and we need a manager role. 
We can do this with the `AccessControl`. Also, we want to use the `LendingStorage` we have declared. 
So we will declare a struct and derive all the needed traits.

```rust
#[ink(storage)]
#[derive(Default, AccessControlStorage, PausableStorage, LendingStorage)]
pub struct LendingContract {
    #[AccessControlStorageField]
    access: AccessControlData,
    #[PausableStorageField]
    pause: PausableData,
    #[LendingStorageField]
    lending: LendingData,
}
```

## Implement traits

We need to "inherit" the implementation of `AccessControll`, `Pausable`, `Lending`, 
`LendingPermissioned` and `LendingPermissionedInternal`.

```rust
impl AccessControl for LendingContract {}

impl Pausable for LendingContract {}

impl Lending for LendingContract {}

impl LendingPermissioned for LendingContract {}

impl LendingPermissionedInternal for LendingContract {
    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {
        let code_hash = self.lending.shares_contract_code_hash;
        let (hash, _) =
            ink_env::random::<ink_env::DefaultEnvironment>(contract_name.as_bytes()).expect("Ger random salt");
        let hash = hash.as_ref();
        let contract = SharesContract::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
            .endowment(10000000000)
            .code_hash(code_hash)
            .salt_bytes(&[hash[0], hash[1], hash[2], hash[3]])
            .instantiate()
            .unwrap();
        contract.to_account_id()
    }
}
```

Now the `LendingContract` has functionality of all that traits.

## Define the constructor

Finally, we will add a constructor, in which we will initiate the admin of 
the contract, to whom we will also grant the manager role declared before, 
and we will also instantiate the `LoanContract` here and store its AccountId 
in `LendingContract`.

```rust
impl LendingContract {
    /// constructor with name and symbol
    #[ink(constructor)]
    pub fn new(code_hash: Hash, nft_code_hash: Hash) -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_admin(caller);
        instance.grant_role(MANAGER, caller).expect("Can not set manager role");
        instance.lending.shares_contract_code_hash = code_hash;
        // instantiate NFT contract and store its account id
        let nft = LoanContract::new()
            .endowment(10000000000)
            .code_hash(nft_code_hash)
            .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate()
            .unwrap();
        instance.lending.loan_account = nft.to_account_id();
        instance
    }
}
```