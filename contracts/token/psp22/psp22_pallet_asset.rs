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

pub use crate::traits::{
    errors::{
        PSP22Error,
        PSP22ReceiverError,
    },
    psp22::*,
};
use crate::psp22::utils::pallet_assets::*;
pub use crate::traits::psp22::psp22asset::*;
use crate::traits::psp22::extensions::mintable::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Flush,
    },
};
pub use derive::*;
use ink_env::{
    CallFlags,
    Error as EnvError,
};
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::Mapping;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP22AssetData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP22AssetData {
    pub asset_id: u32,
    pub origin_type: u8,
}

declare_storage_trait!(PSP22AssetStorage, PSP22AssetData);

impl<T: PSP22AssetStorage + Flush> PSP22Asset for T {
    default fn total_supply(&self) -> Balance {
        PalletAsset::total_supply(self.get().asset_id).unwrap()
    }

    default fn balance_of(&self, owner: AccountId) -> Balance {
        PalletAsset::balance(self.get().asset_id, *owner.as_ref()).unwrap()
    }

    default fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        unimplemented!()
        // self.get().allowances.get((&owner, &spender)).unwrap_or(0)
    }

    default fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
        // let from = Self::env().caller();
        // self._transfer_from_to(from, to, value, data)?;
        // Ok(())
        let origin : OriginType = self.get().origin_type.into();
        let mint_result = PalletAsset::transfer(origin, self.get().asset_id, *to.as_ref(), value.into());
        match mint_result {
            Result::<(), PalletAssetErr>::Ok(_) => Result::<(), PSP22Error>::Ok(()),
            Result::<(), PalletAssetErr>::Err(e) => Result::<(), PSP22Error>::Err(PSP22Error::from(e)),
        }
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let allowance = self.allowance(from, caller);
        // if allowance < value {
        //     return Err(PSP22Error::InsufficientAllowance)
        // }

        // self._transfer_from_to(from, to, value, data)?;
        // self._approve_from_to(from, caller, allowance - value)?;
        // Ok(())
        let origin : OriginType = self.get().origin_type.into();
        let transfer_approved_result = PalletAsset::transfer_approved(origin, self.get().asset_id, *from.as_ref(), *to.as_ref(), value.into());
        match transfer_approved_result {
            Result::<(), PalletAssetErr>::Ok(_) => Result::<(), PSP22Error>::Ok(()),
            Result::<(), PalletAssetErr>::Err(e) => Result::<(), PSP22Error>::Err(PSP22Error::from(e)),
        }
    }

    default fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        // let owner = Self::env().caller();
        // self._approve_from_to(owner, spender, value)?;
        // Ok(())
        let origin : OriginType = self.get().origin_type.into();
        let approve_transfer_result = PalletAsset::approve_transfer(origin, self.get().asset_id, *spender.as_ref(), value.into());
        match approve_transfer_result {
            Result::<(), PalletAssetErr>::Ok(_) => Result::<(), PSP22Error>::Ok(()),
            Result::<(), PalletAssetErr>::Err(e) => Result::<(), PSP22Error>::Err(PSP22Error::from(e)),
        }
    }

    default fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        unimplemented!()
        // let owner = Self::env().caller();
        // self._approve_from_to(owner, spender, self.allowance(owner, spender) + delta_value)?;
        // Ok(())
    }

    default fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        unimplemented!()
        // let owner = Self::env().caller();
        // let allowance = self.allowance(owner, spender);

        // if allowance < delta_value {
        //     return Err(PSP22Error::InsufficientAllowance)
        // }

        // self._approve_from_to(owner, spender, allowance - delta_value)?;
        // Ok(())
    }
}

impl<T: PSP22AssetStorage + Flush> PSP22AssetMintable for T {
    default fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>{
        let mint_result = PalletAsset::mint(self.get().origin_type.into(), self.get().asset_id, *account.as_ref(), amount.into());
        match mint_result {
            Result::<(), PalletAssetErr>::Ok(_) => Result::<(), PSP22Error>::Ok(()),
            Result::<(), PalletAssetErr>::Err(e) => Result::<(), PSP22Error>::Err(PSP22Error::from(e)),
        }
    }
}

// pub trait PSP22Internal {
//     fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);

//     fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

//     fn _balance_of(&self, owner: &AccountId) -> Balance;

//     fn _do_safe_transfer_check(
//         &mut self,
//         from: &AccountId,
//         to: &AccountId,
//         value: &Balance,
//         data: &Vec<u8>,
//     ) -> Result<(), PSP22Error>;

//     fn _transfer_from_to(
//         &mut self,
//         from: AccountId,
//         to: AccountId,
//         amount: Balance,
//         data: Vec<u8>,
//     ) -> Result<(), PSP22Error>;

//     fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<(), PSP22Error>;

//     fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

//     fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
// }

// impl<T: PSP22Storage + Flush> PSP22Internal for T {
//     default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

//     default fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

//     default fn _balance_of(&self, owner: &AccountId) -> Balance {
//         self.get().balances.get(owner).unwrap_or(0)
//     }

//     default fn _do_safe_transfer_check(
//         &mut self,
//         from: &AccountId,
//         to: &AccountId,
//         value: &Balance,
//         data: &Vec<u8>,
//     ) -> Result<(), PSP22Error> {
//         self.flush();
//         let builder = PSP22ReceiverRef::before_received_builder(
//             to,
//             Self::env().caller(),
//             from.clone(),
//             value.clone(),
//             data.clone(),
//         )
//         .call_flags(CallFlags::default().set_allow_reentry(true));
//         let result = match builder.fire() {
//             Ok(result) => {
//                 match result {
//                     Ok(_) => Ok(()),
//                     Err(e) => Err(e.into()),
//                 }
//             }
//             Err(e) => {
//                 match e {
//                     // `NotCallable` means that the receiver is not a contract.

//                     // `CalleeTrapped` means that the receiver has no method called `before_received` or it failed inside.
//                     // First case is expected. Second - not. But we can't tell them apart so it is a positive case for now.
//                     // https://github.com/paritytech/ink/issues/1002
//                     EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
//                     _ => {
//                         Err(PSP22Error::SafeTransferCheckFailed(String::from(
//                             "Error during call to receiver",
//                         )))
//                     }
//                 }
//             }
//         };
//         self.load();
//         result?;
//         Ok(())
//     }

//     default fn _transfer_from_to(
//         &mut self,
//         from: AccountId,
//         to: AccountId,
//         amount: Balance,
//         data: Vec<u8>,
//     ) -> Result<(), PSP22Error> {
//         if from.is_zero() {
//             return Err(PSP22Error::ZeroSenderAddress)
//         }
//         if to.is_zero() {
//             return Err(PSP22Error::ZeroRecipientAddress)
//         }

//         let from_balance = self.balance_of(from);

//         if from_balance < amount {
//             return Err(PSP22Error::InsufficientBalance)
//         }

//         self._before_token_transfer(Some(&from), Some(&to), &amount)?;

//         self._do_safe_transfer_check(&from, &to, &amount, &data)?;
//         self.get_mut().balances.insert(&from, &(from_balance - amount));
//         let to_balance = self._balance_of(&to);
//         self.get_mut().balances.insert(&to, &(to_balance + amount));
//         self._after_token_transfer(Some(&from), Some(&to), &amount)?;
//         self._emit_transfer_event(Some(from), Some(to), amount);

//         Ok(())
//     }

//     default fn _approve_from_to(
//         &mut self,
//         owner: AccountId,
//         spender: AccountId,
//         amount: Balance,
//     ) -> Result<(), PSP22Error> {
//         if owner.is_zero() {
//             return Err(PSP22Error::ZeroSenderAddress)
//         }
//         if spender.is_zero() {
//             return Err(PSP22Error::ZeroRecipientAddress)
//         }

//         self.get_mut().allowances.insert((&owner, &spender), &amount);
//         self._emit_approval_event(owner, spender, amount);
//         Ok(())
//     }

//     default fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
//         if account.is_zero() {
//             return Err(PSP22Error::ZeroRecipientAddress)
//         }

//         self._before_token_transfer(None, Some(&account), &amount)?;
//         let mut new_balance = self.balance_of(account);
//         new_balance += amount;
//         self.get_mut().balances.insert(&account, &new_balance);
//         self.get_mut().supply += amount;
//         self._after_token_transfer(None, Some(&account), &amount)?;
//         self._emit_transfer_event(None, Some(account), amount);

//         Ok(())
//     }

//     default fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
//         if account.is_zero() {
//             return Err(PSP22Error::ZeroRecipientAddress)
//         }

//         let mut from_balance = self.balance_of(account);

//         if from_balance < amount {
//             return Err(PSP22Error::InsufficientBalance)
//         }

//         self._before_token_transfer(Some(&account), None, &amount)?;

//         from_balance -= amount;
//         self.get_mut().balances.insert(&account, &from_balance);
//         self.get_mut().supply -= amount;
//         self._after_token_transfer(Some(&account), None, &amount)?;
//         self._emit_transfer_event(Some(account), None, amount);

//         Ok(())
//     }
// }

// pub trait PSP22Transfer {
//     fn _before_token_transfer(
//         &mut self,
//         _from: Option<&AccountId>,
//         _to: Option<&AccountId>,
//         _amount: &Balance,
//     ) -> Result<(), PSP22Error>;

//     fn _after_token_transfer(
//         &mut self,
//         _from: Option<&AccountId>,
//         _to: Option<&AccountId>,
//         _amount: &Balance,
//     ) -> Result<(), PSP22Error>;
// }

// impl<T> PSP22Transfer for T {
//     default fn _before_token_transfer(
//         &mut self,
//         _from: Option<&AccountId>,
//         _to: Option<&AccountId>,
//         _amount: &Balance,
//     ) -> Result<(), PSP22Error> {
//         Ok(())
//     }

//     default fn _after_token_transfer(
//         &mut self,
//         _from: Option<&AccountId>,
//         _to: Option<&AccountId>,
//         _amount: &Balance,
//     ) -> Result<(), PSP22Error> {
//         Ok(())
//     }
// }
