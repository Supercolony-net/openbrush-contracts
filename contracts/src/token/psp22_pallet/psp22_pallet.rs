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

pub use crate::{
    psp22_pallet,
    traits::psp22::*,
};
pub use pallet_assets_chain_extension::{
    ink::*,
    traits::*,
};
pub use psp22_pallet::Internal as _;

use ink_env::DefaultEnvironment;
use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// Asset id of the token on the pallet.
    pub asset_id: u32,
    /// Default origin of the contract.
    pub origin: Origin,
    /// Extension to interact with `pallet-assets`
    pub pallet_assets: AssetsExtension,
    pub _reserved: Option<()>,
}

impl<T: Storage<Data>> PSP22 for T {
    default fn total_supply(&self) -> Balance {
        let self_ = self.data();
        self_.pallet_assets.total_supply(self_.asset_id)
    }

    default fn balance_of(&self, owner: AccountId) -> Balance {
        let self_ = self.data();
        self_.pallet_assets.balance_of(self_.asset_id, owner)
    }

    default fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        let self_ = self.data();
        self_.pallet_assets.allowance(self_.asset_id, owner, spender)
    }

    default fn transfer(&mut self, to: AccountId, value: Balance, _data: Vec<u8>) -> Result<(), PSP22Error> {
        if value == 0 {
            return Ok(())
        }

        let self_ = self.data();
        self_
            .pallet_assets
            .transfer(self_.origin, self_.asset_id, to.clone(), value)?;
        self._emit_transfer_event(Some(self._sender()), Some(to), value);
        Ok(())
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        if value == 0 {
            return Ok(())
        }

        let self_ = self.data();
        self_
            .pallet_assets
            .transfer_approved(self_.origin, self_.asset_id, from.clone(), to.clone(), value)?;
        self._emit_transfer_event(Some(from), Some(to), value);
        Ok(())
    }

    default fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        if value == 0 {
            return Ok(())
        }

        let caller = self._sender();
        let self_ = self.data();
        if self_.allowance(caller.clone(), spender.clone()) > 0 {
            // First we reset the previous approve and after set a new one.
            self_
                .pallet_assets
                .cancel_approval(self_.origin, self_.asset_id, spender.clone())?;
        }

        self_
            .pallet_assets
            .approve_transfer(self_.origin, self_.asset_id, spender, value)?;
        self._emit_approval_event(caller, spender, value);
        Ok(())
    }

    default fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        if delta_value == 0 {
            return Ok(())
        }

        let self_ = self.data();
        let caller = self_._sender();
        let allowance = self_.allowance(caller.clone(), spender.clone());
        // `approve_transfer` increases by default
        self_
            .pallet_assets
            .approve_transfer(self_.origin, self_.asset_id, spender, delta_value)?;
        self._emit_approval_event(caller, spender, allowance + delta_value);

        Ok(())
    }

    default fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        if delta_value == 0 {
            return Ok(())
        }

        let caller = self._sender();

        let mut allowance = self.allowance(caller.clone(), spender.clone());

        if allowance < delta_value {
            return Err(PSP22Error::InsufficientAllowance)
        }
        allowance -= delta_value;

        self.approve(spender, allowance)?;
        self._emit_approval_event(caller, spender, allowance);

        Ok(())
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);
    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _create(
        &mut self,
        asset_id: u32,
        admin: AccountId,
        min_balance: Balance,
    ) -> Result<(), Error<DefaultEnvironment>>;

    fn _sender(&self) -> AccountId;
}

impl<T: Storage<Data>> Internal for T {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}
    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        let self_ = self.data();
        self_.pallet_assets.mint(self_.asset_id, account.clone(), amount)?;
        self._emit_transfer_event(None, Some(account), amount);
        Ok(())
    }

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        let self_ = self.data();
        self_.pallet_assets.burn(self_.asset_id, account.clone(), amount)?;
        self._emit_transfer_event(Some(account), None, amount);
        Ok(())
    }

    fn _create(
        &mut self,
        asset_id: u32,
        admin: AccountId,
        min_balance: Balance,
    ) -> Result<(), Error<DefaultEnvironment>> {
        self.data().pallet_assets.create(asset_id, admin, min_balance)
    }

    fn _sender(&self) -> AccountId {
        match self.data().origin {
            Origin::Caller => Self::env().caller(),
            Origin::Address => Self::env().account_id(),
        }
    }
}

impl From<Error<DefaultEnvironment>> for PSP22Error {
    fn from(error: Error<DefaultEnvironment>) -> Self {
        match error {
            Error::ContractIsNotAdmin => PSP22Error::Custom(String::from("ContractIsNotAdmin")),
            Error::BalanceLow => PSP22Error::InsufficientBalance,
            Error::NoAccount => PSP22Error::Custom(String::from("NoAccount")),
            Error::NoPermission => PSP22Error::Custom(String::from("NoPermission")),
            Error::Unknown => PSP22Error::Custom(String::from("Unknown")),
            Error::Frozen => PSP22Error::Custom(String::from("Frozen")),
            Error::InUse => PSP22Error::Custom(String::from("InUse")),
            Error::BadWitness => PSP22Error::Custom(String::from("BadWitness")),
            Error::MinBalanceZero => PSP22Error::Custom(String::from("MinBalanceZero")),
            Error::NoProvider => PSP22Error::Custom(String::from("NoProvider")),
            Error::BadMetadata => PSP22Error::Custom(String::from("BadMetadata")),
            Error::Unapproved => PSP22Error::InsufficientAllowance,
            Error::WouldDie => PSP22Error::Custom(String::from("WouldDie")),
            Error::AlreadyExists => PSP22Error::Custom(String::from("AlreadyExists")),
            Error::NoDeposit => PSP22Error::Custom(String::from("NoDeposit")),
            Error::WouldBurn => PSP22Error::Custom(String::from("WouldBurn")),
            Error::AssetPalletInternal => PSP22Error::Custom(String::from("AssetPalletInternal")),
            // All future errors should be `AssetPalletInternal`
            _ => panic!("other error are not supported"),
        }
    }
}
