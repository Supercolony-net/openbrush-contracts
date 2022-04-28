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

/// Extension of [`PSP22`] which allows the beneficiary to extract tokens after given time
use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::{
    string::String,
    vec::Vec,
};

use crate::traits::psp22::PSP22Error;
use ink_lang::ChainExtensionInstance;

pub struct PalletAsset;

impl PalletAsset {
    pub fn create(
        origin_type: OriginType,
        asset_id: u32,
        target_address: [u8; 32],
        min_balance: u128,
    ) -> Result<(), PalletAssetErr> {
        let subject = PalletAssetRequest {
            origin_type,
            asset_id,
            target_address,
            amount: min_balance,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1102u32)
            .input::<PalletAssetRequest>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&subject)?
    }

    pub fn mint(
        origin_type: OriginType,
        asset_id: u32,
        target_address: [u8; 32],
        amount: u128,
    ) -> Result<(), PalletAssetErr> {
        let subject = PalletAssetRequest {
            origin_type,
            asset_id,
            target_address,
            amount,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1103u32)
            .input::<PalletAssetRequest>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&subject)?
    }

    pub fn burn(
        origin_type: OriginType,
        asset_id: u32,
        target_address: [u8; 32],
        amount: u128,
    ) -> Result<(), PalletAssetErr> {
        let subject = PalletAssetRequest {
            origin_type,
            asset_id,
            target_address,
            amount,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1104u32)
            .input::<PalletAssetRequest>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&subject)?
    }

    pub fn transfer(
        origin_type: OriginType,
        asset_id: u32,
        target_address: [u8; 32],
        amount: u128,
    ) -> Result<(), PalletAssetErr> {
        let subject = PalletAssetRequest {
            origin_type,
            asset_id,
            target_address,
            amount,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1105u32)
            .input::<PalletAssetRequest>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&subject)?
    }

    pub fn balance(asset_id: u32, address: [u8; 32]) -> Result<u128, PalletAssetErr> {
        let subject = PalletAssetBalanceRequest { asset_id, address };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1106u32)
            .input::<PalletAssetBalanceRequest>()
            .output::<u128>()
            .handle_error_code::<PalletAssetErr>()
            .call(&subject)
    }

    pub fn total_supply(asset_id: u32) -> Result<u128, PalletAssetErr> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(1107u32)
            .input::<u32>()
            .output::<u128>()
            .handle_error_code::<PalletAssetErr>()
            .call(&asset_id)
    }

    pub fn approve_transfer(
        origin_type: OriginType,
        asset_id: u32,
        target_address: [u8; 32],
        amount: u128,
    ) -> Result<(), PalletAssetErr> {
        let subject = PalletAssetRequest {
            origin_type,
            asset_id,
            target_address,
            amount,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1108u32)
            .input::<PalletAssetRequest>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&subject)?
    }

    pub fn transfer_approved(
        origin_type: OriginType,
        asset_id: u32,
        owner: [u8; 32],
        target_address: [u8; 32],
        amount: u128,
    ) -> Result<(), PalletAssetErr> {
        let subject = PalletAssetRequest {
            origin_type,
            asset_id,
            target_address,
            amount,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(1109u32)
            .input::<([u8; 32], PalletAssetRequest)>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&(owner, subject))?
    }

    pub fn allowance(asset_id: u32, owner: [u8; 32], spender: [u8; 32]) -> Result<u128, PalletAssetErr> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(1110u32)
            .input::<(u32, [u8; 32], [u8; 32])>()
            .output::<u128>()
            .handle_error_code::<PalletAssetErr>()
            .call(&(asset_id, owner, spender))
    }

    // increase or decrease
    pub fn change_allowance(
        asset_id: u32,
        owner: [u8; 32],
        delegate: [u8; 32],
        delta_value: u128,
        is_increase: bool,
    ) -> Result<(), PalletAssetErr> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(1111u32)
            .input::<(u32, [u8; 32], [u8; 32], u128, bool)>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&(asset_id, owner, delegate, delta_value, is_increase))?
    }

    pub fn set_metadata(
        origin_type: OriginType,
        asset_id: u32,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
    ) -> Result<(), PalletAssetErr> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(1112u32)
            .input::<(OriginType, u32, Vec<u8>, Vec<u8>, u8)>()
            .output::<Result<(), PalletAssetErr>>()
            .handle_error_code::<PalletAssetErr>()
            .call(&(origin_type, asset_id, name, symbol, decimals))?
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RequestType {
    Create,
    Mint,
    Burn,
    Transfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OriginType {
    Caller,
    Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PalletAssetRequest {
    pub origin_type: OriginType,
    pub asset_id: u32,
    pub target_address: [u8; 32],
    pub amount: u128,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PalletAssetBalanceRequest {
    pub asset_id: u32,
    pub address: [u8; 32],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PalletAssetErr {
    /// Some error occurred.
    Other,
    /// Failed to lookup some data.
    CannotLookup,
    /// A bad origin.
    BadOrigin,
    /// A custom error in a module.
    Module,
    /// At least one consumer is remaining so the account cannot be destroyed.
    ConsumerRemaining,
    /// There are no providers so the account cannot be created.
    NoProviders,
    /// There are too many consumers so the account cannot be created.
    TooManyConsumers,
    /// An error to do with tokens.
    Token(PalletAssetTokenErr),
    /// An arithmetic error.
    Arithmetic(PalletAssetArithmeticErr),
    // unknown error
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PalletAssetArithmeticErr {
    /// Underflow.
    Underflow,
    /// Overflow.
    Overflow,
    /// Division by zero.
    DivisionByZero,
    // unknown error
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PalletAssetTokenErr {
    /// Funds are unavailable.
    NoFunds,
    /// Account that must exist would die.
    WouldDie,
    /// Account cannot exist with the funds that would be given.
    BelowMinimum,
    /// Account cannot be created.
    CannotCreate,
    /// The asset in question is unknown.
    UnknownAsset,
    /// Funds exist but are frozen.
    Frozen,
    /// Operation is not supported by the asset.
    Unsupported,
    // unknown error
    Unknown,
}

impl From<u8> for OriginType {
    fn from(origin: u8) -> OriginType {
        if origin == 0 {
            OriginType::Caller
        } else {
            OriginType::Address
        }
    }
}

impl From<PalletAssetErr> for PSP22Error {
    fn from(e: PalletAssetErr) -> PSP22Error {
        match e {
            PalletAssetErr::Other => PSP22Error::Custom(String::from("psp22 error")),
            PalletAssetErr::CannotLookup => PSP22Error::Custom(String::from("CannotLookup")),
            PalletAssetErr::BadOrigin => PSP22Error::Custom(String::from("BadOrigin")),
            PalletAssetErr::Module => PSP22Error::Custom(String::from("Module")),
            PalletAssetErr::ConsumerRemaining => PSP22Error::Custom(String::from("ConsumerRemaining")),
            PalletAssetErr::NoProviders => PSP22Error::Custom(String::from("NoProviders")),
            PalletAssetErr::TooManyConsumers => PSP22Error::Custom(String::from("TooManyConsumers")),
            PalletAssetErr::Token(token_err) => PSP22Error::Custom(String::from("Token")),
            PalletAssetErr::Arithmetic(arithmetic_error) => PSP22Error::Custom(String::from("Arithmetic")),
            _ => PSP22Error::Custom(String::from("Unnown")),
        }
    }
}

impl ink_env::chain_extension::FromStatusCode for PalletAssetErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for PalletAssetErr {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
