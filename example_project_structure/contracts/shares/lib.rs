#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract will be used to represent the shares of a user
/// and other instance of this contract will be used to represent
/// the amount of borrowed tokens
#[brush::contract]
pub mod shares {
    use brush::{
        contracts::{
            ownable::*,
            psp22::extensions::{
                burnable::*,
                metadata::*,
                mintable::*,
            },
        },
        modifiers,
    };
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use lending_project::traits::shares::*;

    /// Define the storage for PSP22 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, OwnableStorage, PSP22MetadataStorage)]
    pub struct SharesContract {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[OwnableStorageField]
        ownable: OwnableData,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    // implement PSP22 Trait for our share
    impl PSP22 for SharesContract {}

    // implement Ownable Trait for our share
    impl Ownable for SharesContract {}

    // implement Metadata Trait for our share
    impl PSP22Metadata for SharesContract {}

    // implement Mintable Trait for our share
    impl PSP22Mintable for SharesContract {
        /// override the `mint` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint(account, amount)
        }
    }

    // implement Burnable Trait for our share
    impl PSP22Burnable for SharesContract {
        /// override the `burn` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._burn_from(account, amount)
        }
    }

    // It forces the compiler to check that you implemented all super traits
    impl Shares for SharesContract {}

    impl SharesContract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut SharesContract| {
                let caller = instance.env().caller();
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = 18;
                instance._init_with_owner(caller);
            })
        }
    }
}
