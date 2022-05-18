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
    use contracts::{
        ownable::*,
        diamond::*
    };
    use ink_env::DefaultEnvironment;
    use ink_env::test::DefaultAccounts;
    use contracts::diamond::extensions::diamond_loupe::*;
    use brush::test_utils::{
        accounts,
        change_caller,
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
        ownable: OwnableData
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
        let instance = DiamondContract::new(accounts.alice);
        assert_eq!(instance.owner(), accounts.alice);
    }
}
