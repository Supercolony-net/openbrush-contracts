#![cfg_attr(not(feature = "std"), no_std)]

/// This contract will be used to represent the shares of a user
/// and other instance of this contract will be used to represent
/// the amount of borrowed tokens
#[brush::contract]
pub mod shares {
    use brush::modifiers;
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ink_storage::Lazy;
    use ownable::traits::*;
    use psp22::{
        extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
        traits::*,
    };

    /// Define the storage for PSP22 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, PSP22Storage, OwnableStorage, PSP22MetadataStorage)]
    pub struct Shares {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[OwnableStorageField]
        ownable: OwnableData,
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
    impl PSP22 for Shares {
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

    /// implement Mintable Trait for our coin
    impl PSP22Mintable for Shares {}

    /// implement Burnable Trait for our coin
    impl PSP22Burnable for Shares {}

    /// implement Ownable Trait for our coin
    impl Ownable for Shares {}

    /// implement Metadata Trait for our coin
    impl PSP22Metadata for Shares {}

    impl Shares {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            Lazy::set(&mut instance.metadata.name, name);
            Lazy::set(&mut instance.metadata.symbol, symbol);
            Lazy::set(&mut instance.metadata.decimals, 18);
            instance._init_with_owner(caller);
            instance
        }

        /// override the `mint` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            PSP22Mintable::mint(self, account, amount)
        }

        /// override the `burn` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            PSP22Burnable::burn(self, amount)
        }

        /// override the `burn_from` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            PSP22Burnable::burn_from(self, account, amount)
        }
    }
}
