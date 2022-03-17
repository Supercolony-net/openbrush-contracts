#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22 {
    use brush::contracts::psp22::extensions::metadata::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22 for MyPSP22 {}

    impl PSP22Metadata for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.initialize(total_supply, name, symbol, decimal)
            })
        }

        #[ink(message)]
        pub fn initialize(&mut self, total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) {
            self.metadata.name = name;
            self.metadata.symbol = symbol;
            self.metadata.decimals = decimal;
            MyPSP22::_mint(self, self.env().caller(), total_supply).expect("Should mint");
        }
    }
}
