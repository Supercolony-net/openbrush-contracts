#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod diamond {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::diamond::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        diamond: diamond::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(diamond_facet: FacetCut) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._diamond_cut_facet(&diamond_facet).expect("Init diamond cut");
                instance._init_with_owner(Self::env().caller());
            })
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            self._fallback()
        }
    }
}
