#![cfg_attr(not(feature = "std"), no_std)]

/// This is a simple `PSP-22` which will be used as a collateral token in our lending contract
#[brush::contract]
pub mod collateral {
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ink_storage::Lazy;
    use psp22::{
        extensions::metadata::*,
        traits::*,
    };

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22MetadataStorage)]
    pub struct Collateral {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    /// Event emitted when a token approval occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// implement PSP22 Trait for our coin
    impl PSP22 for Collateral {
        fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                spender: _spender,
                value: _amount,
            });
        }
    }

    /// implement Metadata Trait for our coin
    impl PSP22Metadata for Collateral {}

    impl Collateral {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            Lazy::set(&mut instance.metadata.name, Some(String::from("Collateral")));
            Lazy::set(&mut instance.metadata.symbol, Some(String::from("COL")));
            Lazy::set(&mut instance.metadata.decimals, 18);
            let total_supply = 1_000_000 * 10_u128.pow(18);
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }
    }
}
