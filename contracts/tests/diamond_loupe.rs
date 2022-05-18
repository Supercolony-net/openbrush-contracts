// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(min_specialization)]
#[cfg(feature = "diamond")]
#[brush::contract]
mod diamond {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::{
        diamond::{
            extensions::diamond_loupe::*,
            *,
        },
        ownable::*,
    };
    use ink_env::{
        test::DefaultAccounts,
        DefaultEnvironment,
    };
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, DiamondStorage, DiamondLoupeStorage, OwnableStorage)]
    pub struct DiamondContract {
        #[DiamondStorageField]
        diamond: DiamondData,
        #[DiamondLoupeStorageField]
        diamond_loupe: DiamondLoupeData,
        #[OwnableStorageField]
        ownable: OwnableData,
    }
    impl DiamondContract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(owner);
            })
        }
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            self._fallback()
        }
    }
    impl Ownable for DiamondContract {}
    impl Diamond for DiamondContract {}
    impl DiamondLoupe for DiamondContract {}
    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();
        accounts
    }
    #[ink::test]
    fn constructor_works() {
        let accounts = setup();
        let diamond = DiamondContract::new(accounts.alice);
        // assert
        assert_eq!(diamond.owner(), accounts.alice);
    }

    #[ink::test]
    fn facets_empty_works() {
        let accounts = setup();
        let diamond = DiamondContract::new(accounts.alice);
        // assert
        assert_eq!(diamond.facets(), vec![]);
    }

    #[ink::test]
    fn facets_not_empty_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        // act
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(diamond.facets(), vec![facet_cut]);
    }

    #[ink::test]
    fn facet_function_selectors_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        let selectors: Vec<Selector> = vec![];
        assert_eq!(diamond.facet_function_selectors(facet_cut.hash), selectors);
        // act
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(diamond.facet_function_selectors(facet_cut.hash), facet_cut.selectors);
    }

    #[ink::test]
    fn facet_code_hashes_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);
        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        assert_eq!(diamond.facet_code_hashes(), vec![]);
        // act
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(diamond.facet_code_hashes(), vec![facet_cut.hash]);
    }

    #[ink::test]
    fn facet_code_hash_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        assert_eq!(diamond.facet_code_hash(facet_cut.selectors[0]), Option::None);
        // act
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(
            diamond.facet_code_hash(facet_cut.selectors[0]),
            Option::Some(facet_cut.hash)
        );
    }
}
