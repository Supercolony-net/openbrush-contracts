#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_upgradeable {
    use brush::contracts::psp22::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_lang as ink;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data
    }

    impl PSP22 for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut MyPSP22| {
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }
        
        #[ink(message)]
        pub fn initialize(&mut self, total_supply: Balance) {
            PSP22Internal::_mint(self, self.env().caller(), total_supply).expect("Should mint")
        }
    }

    #[ink::test]
    fn total_supply_works() {
        // Constructor works.
        let mut psp22 = MyPSP22::new(0);
        assert_eq!(psp22.total_supply(), 0);
        psp22.initialize(100);
        // Get the token total supply.
        assert_eq!(psp22.total_supply(), 100);
    }
}

