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
#[cfg(feature = "psp22")]
#[openbrush::contract]
mod psp22_flashmint {
    use ink_lang as ink;
    use ink_lang::codegen::Env;
    use openbrush::{
        contracts::psp22::extensions::flashmint::*,
        test_utils::accounts,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct PSP22FlashMintStruct {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for PSP22FlashMintStruct {}

    // we get rid of cross contract call in test
    impl PSP22FlashLenderInternal for PSP22FlashMintStruct {
        // we will add 1% fee to the amount
        fn _get_fee(&self, amount: Balance) -> Balance {
            amount / 100
        }

        fn _on_flashloan(
            &mut self,
            _receiver_account: AccountId,
            _token: AccountId,
            _fee: Balance,
            _amount: Balance,
            _data: Vec<u8>,
        ) -> Result<(), FlashLenderError> {
            Ok(())
        }
    }

    impl FlashLender for PSP22FlashMintStruct {}

    impl PSP22FlashMintStruct {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(Self::env().caller(), total_supply).is_ok());
            instance
        }
    }

    #[ink::test]
    fn new_works() {
        let total_supply = 1000;
        let mut instance = PSP22FlashMintStruct::new(total_supply);

        // max flashloan is max - total supply
        assert_eq!(
            instance.max_flashloan(instance.env().account_id()),
            Balance::MAX - total_supply
        );
        // flash fee is 1/100 of amount
        assert_eq!(instance.flash_fee(instance.env().account_id(), 100), Ok(1));
        // wrong token
        assert_eq!(instance.max_flashloan(AccountId::from([0x10; 32])), 0);
        // flash fee on wrong token throws error
        assert_eq!(
            instance.flash_fee(AccountId::from([0x10; 32]), 100),
            Err(FlashLenderError::WrongTokenAddress)
        );
    }

    #[ink::test]
    fn flashloan_works() {
        let total_supply = 1000;
        let mut instance = PSP22FlashMintStruct::new(total_supply);

        let receiver = AccountId::from([0x1; 32]);
        let token = instance.env().account_id();
        let loan_amount = 100;
        let fee = instance._get_fee(loan_amount);

        assert!(instance.approve(token, loan_amount + fee).is_ok());
        assert!(instance
            .flashloan(receiver, token, loan_amount, Vec::<u8>::new())
            .is_ok());
        assert_eq!(instance.total_supply(), total_supply - fee);
        assert_eq!(instance.balance_of(accounts().alice), total_supply - fee);
    }

    #[ink::test]
    fn no_allowance_for_fee() {
        let total_supply = 1000;
        let mut instance = PSP22FlashMintStruct::new(total_supply);

        let receiver = AccountId::from([0x1; 32]);
        let token = instance.env().account_id();
        let loan_amount = 100;

        assert_eq!(
            instance.flashloan(receiver, token, loan_amount, Vec::<u8>::new()),
            Err(FlashLenderError::AllowanceDoesNotAllowRefund)
        );
    }
}
