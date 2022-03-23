#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_facet {
    use brush::contracts::psp22::*;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage)]
    pub struct PSP22Facet {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22Internal for PSP22Facet {}

    impl PSP22 for PSP22Facet {}

    impl PSP22Facet {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PSP22Facet| {
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }
    }
}
