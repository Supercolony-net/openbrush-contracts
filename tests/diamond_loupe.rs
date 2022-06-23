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
#[openbrush::contract]
mod diamond {
    use ink_env::{
        test::DefaultAccounts,
        DefaultEnvironment,
    };
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::diamond::{
            extensions::diamond_loupe::*,
            FacetCut,
            *,
        },
        test_utils::accounts,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, DiamondStorage, DiamondLoupeStorage)]
    pub struct DiamondContract {
        #[DiamondStorageField]
        diamond: DiamondData,
        #[DiamondLoupeStorageField]
        diamond_loupe: DiamondLoupeData,
    }

    impl OwnableStorage for DiamondContract {
        type Data = OwnableData;
        fn get(&self) -> &OwnableData {
            &self.diamond.ownable
        }

        fn get_mut(&mut self) -> &mut OwnableData {
            &mut self.diamond.ownable
        }
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
    fn hash_is_clear_should_fails() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [0u8; 32].into(),
            selectors: vec![[0u8; 4]],
        };
        // assert
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Err(DiamondError::EmptyCodeHash)
        );
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

    #[ink::test]
    fn facets_add_selectors_works() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let mut facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        facet_cut.selectors.push([2u8; 4]);
        // act
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(diamond.facets()[0].selectors.len(), 2);
        assert_eq!(diamond.facets(), vec![facet_cut.clone()]);
    }

    #[ink::test]
    fn facets_remove_selectors_works() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let mut facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4], [2u8; 4], [3u8; 4]],
        };
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        facet_cut.selectors.pop();
        assert_eq!(facet_cut.selectors.len(), 2);
        // act
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(diamond.facets()[0].selectors.len(), 2);
        assert_eq!(diamond.facets(), vec![facet_cut.clone()]);
    }

    #[ink::test]
    fn facets_edit_selectors_works() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let mut facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4], [2u8; 4], [3u8; 4]],
        };

        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        assert_eq!(diamond.facets()[0].selectors.len(), 3);
        // act
        facet_cut.selectors[2] = [4u8; 4];
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        // assert
        assert_eq!(diamond.facets()[0].selectors.len(), 3);
        assert_eq!(diamond.facets(), vec![facet_cut.clone()]);
        assert_eq!(diamond.facets()[0].selectors[2], [4u8; 4]);
    }

    #[ink::test]
    fn facets_add_facetcut_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        assert_eq!(diamond.facets().len(), 0);
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        assert_eq!(diamond.facets().len(), 1);
        // act
        let facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[2u8; 4]],
        };
        let v = vec![facet_cut, facet_cut_new];
        assert_eq!(diamond.diamond_cut(v.clone(), Option::None), Result::Ok(()));
        // assert
        assert_eq!(diamond.facets().len(), 2);
        assert_eq!(diamond.facets(), v);
    }

    #[ink::test]
    fn facets_add_facetcut_should_fail_replace_existing() {
        // arrange
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        assert_eq!(diamond.facets().len(), 0);
        assert_eq!(
            diamond.diamond_cut(vec![facet_cut.clone()], Option::None),
            Result::Ok(())
        );
        assert_eq!(diamond.facets().len(), 1);
        let facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        let v = vec![facet_cut.clone(), facet_cut_new];
        // act
        let result = diamond.diamond_cut(v.clone(), Option::None);
        // assert
        assert_eq!(result, Err(DiamondError::ReplaceExisting([1u8; 32].into())));
        assert_eq!(diamond.facets().len(), 1);
        assert_eq!(diamond.facets(), vec![facet_cut]);
    }

    #[ink::test]
    fn facets_edit_facetcut_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [1u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        let mut facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[2u8; 4]],
        };
        let mut v = vec![facet_cut.clone(), facet_cut_new.clone()];
        assert_eq!(diamond.facets().len(), 0);
        assert_eq!(diamond.diamond_cut(v.clone(), Option::None), Result::Ok(()));
        assert_eq!(diamond.facets().len(), 2);
        facet_cut_new.selectors = vec![[5u8; 4], [6u8; 4]];
        v[1] = facet_cut_new;
        // act
        assert_eq!(diamond.diamond_cut(v.clone(), Option::None), Result::Ok(()));
        // assert
        assert_eq!(diamond.facets().len(), 2);
        assert_eq!(diamond.facets(), v);
    }

    #[ink::test]
    fn facets_remove_facetcut_works() {
        let accounts = setup();
        let mut diamond = DiamondContract::new(accounts.alice);

        let facet_cut = FacetCut {
            hash: [3u8; 32].into(),
            selectors: vec![[1u8; 4]],
        };
        let facet_cut_new = FacetCut {
            hash: [2u8; 32].into(),
            selectors: vec![[2u8; 4]],
        };
        let mut v = vec![facet_cut.clone(), facet_cut_new];
        assert_eq!(diamond.facets().len(), 0);
        assert_eq!(diamond.diamond_cut(v.clone(), Option::None), Result::Ok(()));
        assert_eq!(diamond.facets().len(), 2);
        // act
        v[1].selectors = vec![];
        assert_eq!(diamond.diamond_cut(v, Option::None), Result::Ok(()));
        // assert
        assert_eq!(diamond.facets().len(), 1);
        assert_eq!(diamond.facets(), vec![facet_cut]);
    }
}
