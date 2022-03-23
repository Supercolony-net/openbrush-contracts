#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod diamond {
    use brush::contracts::diamond::*;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, DiamondStorage)]
    pub struct DiamondContract {
        #[DiamondStorageField]
        diamond: DiamondData,
    }

    impl DiamondContract {
        #[ink(constructor)]
        pub fn new(diamond_cut: Vec<FacetCut>, owner: AccountId, diamond_hash: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance
                    ._diamond_cut(diamond_cut, None)
                    .expect("Error while cutting the diamond :(");
                instance.diamond.ownable.owner = owner;
                instance.diamond.self_hash = diamond_hash;
            })
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            self._fallback();
        }
    }

    impl Diamond for DiamondContract {}
}
