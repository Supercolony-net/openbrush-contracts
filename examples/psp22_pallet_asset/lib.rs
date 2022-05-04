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
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::{
    string::String,
    vec::Vec,
};

use crate::pallet_assets::*;
use brush::{
    contracts::{
        psp22::{
            extensions::{
                burnable::*,
                metadata::*,
                mintable::*,
            },
            psp22_pallet_asset::*,
            utils::*,
            PSP22Error,
            *,
        },
        traits::psp22::psp22asset::PSP22Asset,
    },
    modifiers,
};
use ink_lang::ChainExtensionInstance;

#[brush::contract]
mod my_psp22_pallet_asset {
    use crate::*;
    use brush::contracts::{
        psp22::{
            psp22_pallet_asset::*,
            *,
        },
        traits::psp22::psp22asset::*,
    };
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22AssetStorage)]
    pub struct MyPSP22 {
        #[PSP22AssetStorageField]
        psp22: PSP22AssetData,
    }

    impl PSP22Asset for MyPSP22 {}

    impl PSP22AssetMintable for MyPSP22 {}

    impl PSP22AssetBurnable for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(
            origin_type: OriginType,
            asset_id: u32,
            target_address: [u8; 32],
            min_balance: u128,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut MyPSP22| {
                instance._create(origin_type, asset_id, target_address, min_balance).expect("should create");
                instance._set_metadata(name, symbol, decimals).expect("should set metadata");
            })
        }

        // 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
        #[ink(message)]
        pub fn get_address(&self) -> [u8; 32] {
            let caller = self.env().caller();
            *caller.as_ref()
        }

        #[ink(message)]
        pub fn set_metadata(
            &self,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Result<(), PSP22Error> {
            self._set_metadata(name, symbol, decimals)
        }

        #[ink(message)]
        pub fn asset_name(&self) -> Option<String> {
            self.token_name()
        }

        #[ink(message)]
        pub fn asset_symbol(&self) -> Option<String> {
            self.token_symbol()
        }

        #[ink(message)]
        pub fn asset_decimals(&self) -> u8 {
            self.token_decimals()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn init_works() {
            // given
            struct CreateAssetExtension;
            impl ink_env::test::ChainExtension for CreateAssetExtension {
                /// The static function id of the chain extension.
                fn func_id(&self) -> u32 {
                    1102
                }

                fn call(&mut self, _input: &[u8], output: &mut Vec<u8>) -> u32 {
                    let mut input = _input;
                    // let r :Result<PalletAssetRequest, scale::Error> = scale::Decode::decode(&mut input);
                    // assert!(r.is_err());

                    let create_result = Result::<(), PalletAssetErr>::Ok(());
                    // let create_result = Result::<(), PalletAssetErr>::Err(PalletAssetErr::Other);
                    scale::Encode::encode_to(&create_result, output);
                    0
                }
            }
            // arrange
            ink_env::test::register_chain_extension(CreateAssetExtension);
            // origin_type: OriginType, asset_id: u32, target_address: [u8; 32], min_balance: u128
            let mut my_psp22 = MyPSP22::new(OriginType::Caller, 10, [1u8; 32], 1);
            // assert
            // assert_eq!(my_psp22.balance_pallet_asset(b.asset_id, b.address), 99);
        }

        // #[ink::test]
        // fn chain_extension_balance_works() {
        //     // given
        //     struct MockedBalanceExtension;
        //     impl ink_env::test::ChainExtension for MockedBalanceExtension {
        //         /// The static function id of the chain extension.
        //         fn func_id(&self) -> u32 {
        //             1106
        //         }

        //         fn call(&mut self, _input: &[u8], output: &mut Vec<u8>) -> u32 {
        //             let b: u128 = 99;
        //             scale::Encode::encode_to(&b, output);
        //             0
        //         }
        //     }
        //     // arrange
        //     ink_env::test::register_chain_extension(MockedBalanceExtension);
        //     let mut my_psp22 = MyPSP22::new(100);
        //     let b = PalletAssetBalanceRequest {
        //         asset_id: 1,
        //         address: [1; 32],
        //     };
        //     // assert
        //     assert_eq!(my_psp22.balance_pallet_asset(b.asset_id, b.address), 99);
        // }
    }

    // Here we define the operations to interact with the Substrate runtime.
    // #[ink::chain_extension]
    // pub trait PalletAssetExtension {
    // type ErrorCode = PalletAssetErr;
    //
    // #[ink(extension = 1102, returns_result = false)]
    // fn create(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;
    //
    // #[ink(extension = 1103, returns_result = false)]
    // fn mint(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;
    //
    // #[ink(extension = 1104, returns_result = false)]
    // fn burn(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;
    //
    // #[ink(extension = 1105, returns_result = false)]
    // fn transfer(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;
    //
    // #[ink(extension = 1106, returns_result = false)]
    // fn balance(subject: PalletAssetBalanceRequest) ->  u128;
    // }
}

// use frame_support::log::{
// error,
// trace,
// };
//
// use pallet_contracts::chain_extension::{
// ChainExtension,
// Environment,
// Ext,
// InitState,
// RetVal,
// SysConfig,
// UncheckedFrom,
// };
//
// use sp_runtime::DispatchError;
// use sp_runtime::TokenError;
// use sp_runtime::ArithmeticError;
//
// use sp_runtime::MultiAddress;
// pub struct PalletAssetsExtention;
//
// struct Origin{}
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode,  MaxEncodedLen)]
// enum OriginType{
// Caller,
// Address
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
// struct PalletAssetRequest{
// origin_type: OriginType,
// asset_id : u32,
// target_address : [u8; 32],
// amount : u128
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
// struct PalletAssetBalanceRequest{
// asset_id : u32,
// address : [u8; 32],
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
// pub enum PalletAssetErr {
// Some error occurred.
// Other,
// Failed to lookup some data.
// CannotLookup,
// A bad origin.
// BadOrigin,
// A custom error in a module.
// Module,
// At least one consumer is remaining so the account cannot be destroyed.
// ConsumerRemaining,
// There are no providers so the account cannot be created.
// NoProviders,
// There are too many consumers so the account cannot be created.
// TooManyConsumers,
// An error to do with tokens.
// Token(PalletAssetTokenErr),
// An arithmetic error.
// Arithmetic(PalletAssetArithmeticErr),
// unknown error
// Unknown,
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
// pub enum PalletAssetArithmeticErr {
// Underflow.
// Underflow,
// Overflow.
// Overflow,
// Division by zero.
// DivisionByZero,
// unknown error
// Unknown,
//
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
// pub enum PalletAssetTokenErr {
// Funds are unavailable.
// NoFunds,
// Account that must exist would die.
// WouldDie,
// Account cannot exist with the funds that would be given.
// BelowMinimum,
// Account cannot be created.
// CannotCreate,
// The asset in question is unknown.
// UnknownAsset,
// Funds exist but are frozen.
// Frozen,
// Operation is not supported by the asset.
// Unsupported,
// unknown error
// Unknown,
// }
//
// impl From<DispatchError> for PalletAssetErr {
// fn from(e: DispatchError) -> Self {
// match e{
// DispatchError::Other(_) => PalletAssetErr::Other,
// DispatchError::CannotLookup => PalletAssetErr::CannotLookup,
// DispatchError::BadOrigin => PalletAssetErr::BadOrigin,
// DispatchError::Module(_) => PalletAssetErr::Module,
// DispatchError::ConsumerRemaining => PalletAssetErr::ConsumerRemaining,
// DispatchError::NoProviders => PalletAssetErr::NoProviders,
// DispatchError::TooManyConsumers => PalletAssetErr::TooManyConsumers,
// DispatchError::Token(token_err) => PalletAssetErr::Token(PalletAssetTokenErr::from(token_err)),
// DispatchError::Arithmetic(arithmetic_error) => PalletAssetErr::Arithmetic(PalletAssetArithmeticErr::from(arithmetic_error)),
// _ => PalletAssetErr::Unknown,
// }
// }
// }
//
// impl From<ArithmeticError> for PalletAssetArithmeticErr {
// fn from(e: ArithmeticError) -> Self {
// match e{
// ArithmeticError::Underflow => PalletAssetArithmeticErr::Underflow,
// ArithmeticError::Overflow => PalletAssetArithmeticErr::Overflow,
// ArithmeticError::DivisionByZero => PalletAssetArithmeticErr::DivisionByZero,
// _ => PalletAssetArithmeticErr::Unknown,
// }
// }
// }
//
// impl From<TokenError> for PalletAssetTokenErr {
// fn from(e: TokenError) -> Self {
// match e{
// TokenError::NoFunds => PalletAssetTokenErr::NoFunds,
// TokenError::WouldDie => PalletAssetTokenErr::WouldDie,
// TokenError::BelowMinimum => PalletAssetTokenErr::BelowMinimum,
// TokenError::CannotCreate => PalletAssetTokenErr::CannotCreate,
// TokenError::UnknownAsset => PalletAssetTokenErr::UnknownAsset,
// TokenError::Frozen => PalletAssetTokenErr::Frozen,
// TokenError::Unsupported => PalletAssetTokenErr::Unsupported,
// _ => PalletAssetTokenErr::Unknown,
// }
// }
// }
//
//
//
//
// impl ChainExtension<Runtime> for PalletAssetsExtention {
//
// fn call<E: Ext>(
// func_id: u32,
// mut env: Environment<E, InitState>,
// ) -> Result<RetVal, DispatchError>
// where
// <E::T as SysConfig>::AccountId:
// UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
// {
// match func_id {
//
// create
//
// 1101 => {
// let ext = env.ext();
// let address : &<<E as Ext>::T as SysConfig>::AccountId = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let caller_accountId = AccountId::decode(&mut caller_ref).unwrap();
//
// use frame_support::dispatch::DispatchError;
// use frame_support::dispatch::DispatchResult;
//
// let mut address_ref = caller.as_ref();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
// let create_result = pallet_assets::Pallet::<Runtime>::
// create(Origin::signed(caller_accountId.clone()), 1, MultiAddress::Id(address_account.clone()), 1);
// match create_result {
// DispatchResult::Ok(_) => error!("OK"),
// DispatchResult::Err(e) => error!("{:#?}", e)
// }
// enum (caller, address_account)
// asset id
// let mint_result =
// pallet_assets::Pallet::<Runtime>::
// mint(Origin::signed(caller_accountId), 1, MultiAddress::Id(address_account), 10);
// match mint_result {
// DispatchResult::Ok(_) => error!("OK"),
// DispatchResult::Err(e) => error!("{:#?}", e)
// }
//
// let r = pallet_assets::Pallet::<Runtime>::total_supply(1);
// error!("total_supply: {:}", r);
// return Err(DispatchError::Other("Unimplemented func_id"))
// let mut env = env.buf_in_buf_out();
// let arg: [u8; 32] = env.read_as()?;
// let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
// let random_slice = random_seed.encode();
// trace!(
//     target: "runtime",
//     "[ChainExtension]|call|func_id:{:}",
//     func_id
// );
// env.write(&arg, false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call random")
// })?;
// }
//
// 1100 => {
// let ext = env.ext();
// let mut env = env.buf_in_buf_out();
// error!("ERROR test");
// let err = Result::<u8,PalletAssetErr>::Err(PalletAssetErr::Other);
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call test")
// })?;
// }
//
// create
// 1102 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let create_asset: PalletAssetRequest = env.read_as()?;
//
// let origin_address = match create_asset.origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
//
// let mut vec = &create_asset.target_address.to_vec()[..];
// let admin_address = AccountId::decode(&mut vec).unwrap();
// let create_result = pallet_assets::Pallet::<Runtime>::
// create(Origin::signed(origin_address),
// create_asset.asset_id,
// MultiAddress::Id(admin_address),
// create_asset.amount);
//
// error!("create input {:#?}", create_asset);
// error!("create output {:#?}", create_result);
// match create_result {
// DispatchResult::Ok(_) => {
// error!("OK create");
// let err = Result::<(),PalletAssetErr>::Ok(());
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call create")
// })?;
// }
// DispatchResult::Err(e) => {
// error!("ERROR create");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call create")
// })?;
// }
// }
//
// }
//
// mint
// 1103 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let mint_asset_request: PalletAssetRequest = env.read_as()?;
//
// let origin_address = match mint_asset_request.origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
//
// let mut vec = &mint_asset_request.target_address.to_vec()[..];
// let beneficiary_address = AccountId::decode(&mut vec).unwrap();
// let mint_result = pallet_assets::Pallet::<Runtime>::
// mint(Origin::signed(origin_address),
// mint_asset_request.asset_id,
// MultiAddress::Id(beneficiary_address),
// mint_asset_request.amount);
//
// error!("mint input {:#?}", mint_asset_request);
// error!("mint output {:#?}", mint_result);
// match mint_result {
// DispatchResult::Ok(_) => {
// error!("OK mint")
// },
// DispatchResult::Err(e) => {
// error!("ERROR mint");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call mint")
// })?;
// }
// }
// }
//
// burn
// 1104 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let burn_asset_request: PalletAssetRequest = env.read_as()?;
//
// let origin_address = match burn_asset_request.origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
//
// let mut vec = &burn_asset_request.target_address.to_vec()[..];
// let who_address = AccountId::decode(&mut vec).unwrap();
// let burn_result = pallet_assets::Pallet::<Runtime>::
// burn(Origin::signed(origin_address),
// burn_asset_request.asset_id,
// MultiAddress::Id(who_address),
// burn_asset_request.amount);
//
// error!("burn request {:#?}", burn_asset_request);
// error!("burn result {:#?}", burn_result);
// match burn_result {
// DispatchResult::Ok(_) => {
// error!("OK burn")
// }
// DispatchResult::Err(e) => {
// error!("ERROR burn");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call burn")
// })?;
// }
// }
// }
//
// transfer
// 1105 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let transfer_asset_request: PalletAssetRequest = env.read_as()?;
//
// let origin_address = match transfer_asset_request.origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
//
// let mut vec = &transfer_asset_request.target_address.to_vec()[..];
// let target_address = AccountId::decode(&mut vec).unwrap();
// let tranfer_result = pallet_assets::Pallet::<Runtime>::
// transfer(Origin::signed(origin_address),
// transfer_asset_request.asset_id,
// MultiAddress::Id(target_address),
// transfer_asset_request.amount);
//
// trace!("transfer request {:#?}", transfer_asset_request);
// trace!("transfer result {:#?}", tranfer_result);
// match tranfer_result {
// DispatchResult::Ok(_) => {
// error!("OK transfer");
//
// write buffer as responce for smart contract
// let b = [1u8;32];
// env.write(&b, false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call random")
// })?;
// /
// }
// DispatchResult::Err(e) => {
// error!("ERROR transfer");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call burn")
// })?;
// }
// }
// }
//
// balance
// 1106 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let balance_asset_request: PalletAssetBalanceRequest = env.read_as()?;
//
//
// let mut vec = &balance_asset_request.address.to_vec()[..];
// let balance_of_address = AccountId::decode(&mut vec).unwrap();
// let balance_result : Balance = pallet_assets::Pallet::<Runtime>::
// balance(balance_asset_request.asset_id,balance_of_address);
//
// error!("OK! balance_of : {:#?}", balance_result);
// error!("{:#?}", balance_asset_request);
//
// let b = balance_result.to_be_bytes();
// write buffer as responce for smart contract
// env.write(&b, false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call balance")
// })?;
// }
//
// total_supply
// 1107 => {
// let ext = env.ext();
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let asset_id: u32 = env.read_as()?;
//
// let total_supply : Balance = pallet_assets::Pallet::<Runtime>::total_supply(asset_id);
//
// error!("total_supply : {:#?}", total_supply);
// error!("total_supply asset_id {:#?}", asset_id);
//
// let b = total_supply.to_be_bytes();
// write buffer as responce for smart contract
// env.write(&b, false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call total_supply")
// })?;
// }
//
// approve_transfer
// 1108 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let approve_transfer_request: PalletAssetRequest = env.read_as()?;
//
// let origin_address = match approve_transfer_request.origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
//
// let mut vec = &approve_transfer_request.target_address.to_vec()[..];
// let target_address = AccountId::decode(&mut vec).unwrap();
// let approve_transfer_result = pallet_assets::Pallet::<Runtime>::
// approve_transfer(Origin::signed(origin_address),
// approve_transfer_request.asset_id,
// MultiAddress::Id(target_address),
// approve_transfer_request.amount);
//
// trace!("approve_transfer request {:#?}", approve_transfer_request);
// trace!("approve_transfer result {:#?}", approve_transfer_result);
// match approve_transfer_result {
// DispatchResult::Ok(_) => {
// error!("OK approve_transfer")
// }
// DispatchResult::Err(e) => {
// error!("ERROR approve_transfer");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call 'approve transfer'")
// })?;
// }
// }
// }
//
// transfer_approved
// 1109 => {
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let approve_transfer_request: ([u8; 32], PalletAssetRequest) = env.read_as()?;
// let owner = approve_transfer_request.0;
// let transfer_approved_request = approve_transfer_request.1;
//
// let origin_address = match transfer_approved_request.origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
//
// let mut vec = &owner.to_vec()[..];
// let owner_address = AccountId::decode(&mut vec).unwrap();
//
// let mut vec = &transfer_approved_request.target_address.to_vec()[..];
// let target_address = AccountId::decode(&mut vec).unwrap();
//
// let transfer_approved_result = pallet_assets::Pallet::<Runtime>::
// transfer_approved(Origin::signed(origin_address),
// transfer_approved_request.asset_id,
// MultiAddress::Id(owner_address),
// MultiAddress::Id(target_address),
// transfer_approved_request.amount);
//
// trace!("transfer_approved request {:#?}", transfer_approved_request);
// trace!("transfer_approved result {:#?}", transfer_approved_result);
// match transfer_approved_result {
// DispatchResult::Ok(_) => {
// error!("OK transfer_approved")
// }
// DispatchResult::Err(e) => {
// error!("ERROR transfer_approved");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call 'transfer approved'")
// })?;
// }
// }
// }
//
// allowance
// 1110 => {
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let allowance_request: (u32, [u8; 32], [u8; 32]) = env.read_as()?;
// let asset_id = allowance_request.0;
//
// let owner = allowance_request.1;
// let delegate = allowance_request.2;
//
// let mut vec = &owner.to_vec()[..];
// let owner_address = AccountId::decode(&mut vec).unwrap();
//
// let mut vec = &delegate.to_vec()[..];
// let delegate_address = AccountId::decode(&mut vec).unwrap();
//
// use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::fungibles::approvals::Inspect;
// let allowance :u128 = Assets::allowance(asset_id, &owner_address, &delegate_address);
//
// trace!("allowance request {:#?}", allowance_request);
// trace!("allowance result {:#?}", allowance);
// let b = allowance.to_be_bytes();
// write buffer as responce for smart contract
// env.write(&b, false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call balance")
// })?;
// }
//
// increase_allowance/decrease_allowance
// 1111 => {
// use frame_support::dispatch::DispatchResult;
//
// let mut env = env.buf_in_buf_out();
// let request: (u32, [u8; 32], [u8; 32], u128, bool) = env.read_as()?;
// let (asset_id, owner, delegate, amount, is_increase) = request;
//
// let mut vec = &owner.to_vec()[..];
// let owner_address = AccountId::decode(&mut vec).unwrap();
//
// let mut vec = &delegate.to_vec()[..];
// let delegate_address = AccountId::decode(&mut vec).unwrap();
//
// use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::fungibles::approvals::Inspect;
// let allowance :u128 = Assets::allowance(asset_id, &owner_address, &delegate_address);
//
// let new_allowance =
// if is_increase {allowance + amount}
// else {
// if allowance < amount  { 0 }
// else {allowance - amount}
// };
//
// let cancel_approval_result = pallet_assets::Pallet::<Runtime>::
// cancel_approval(Origin::signed(owner_address.clone()),
// asset_id,
// MultiAddress::Id(delegate_address.clone()));
// match cancel_approval_result {
// DispatchResult::Ok(_) => {
// error!("OK cancel_approval")
// }
// DispatchResult::Err(e) => {
// error!("ERROR cancel_approval");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call 'approve transfer'")
// })?;
// }
// }
//
// if cancel_approval_result.is_ok(){
// let approve_transfer_result = pallet_assets::Pallet::<Runtime>::
// approve_transfer(Origin::signed(owner_address),
// asset_id,
// MultiAddress::Id(delegate_address),
// new_allowance);
//
// error!("old allowance {}", allowance);
// error!("new allowance {}", new_allowance);
// error!("increase_allowance input {:#?}", request);
// error!("increase_allowance output {:#?}", approve_transfer_result);
// match approve_transfer_result {
// DispatchResult::Ok(_) => {
// error!("OK increase_allowance")
// }
// DispatchResult::Err(e) => {
// error!("ERROR increase_allowance");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call 'approve transfer'")
// })?;
// }
// }
// }
// }
//
// set_metadata
// 1112 => {
// use frame_support::dispatch::DispatchResult;
// let ext = env.ext();
// let address  = ext.address();
// let caller = ext.caller();
// let mut caller_ref = caller.as_ref();
// let mut address_ref = address.as_ref();
// let caller_account = AccountId::decode(&mut caller_ref).unwrap();
// let address_account = AccountId::decode(&mut address_ref).unwrap();
//
//
// let mut env = env.buf_in_buf_out();
// let input: (OriginType, u32, [u8;32], [u8;32], u8) = env.read_as()?;
// let input: (OriginType, u32, Vec<u8>, Vec<u8>, u8) = env.read_as_unbounded(env.in_len())?;
// let (origin_type, asset_id, name, symbol, decimals) = input;
//
// let origin_address = match origin_type {
// OriginType::Caller => {
// caller_account
// },
// OriginType::Address => {
// address_account
// },
// };
// error!("name metadata input {:#?}", name.clone());
// error!("symbol metadata input {:#?}", symbol.clone());
// error!("decimals metadata input {}", decimals);
//
// let result = pallet_assets::Pallet::<Runtime>::
// set_metadata(Origin::signed(origin_address),asset_id,name, symbol, decimals);
//
// error!("set_metadata : {:#?}", result);
// error!("set_metadata input {:#?}", input);
//
// match result {
// DispatchResult::Ok(_) => {
// error!("OK set_metadata");
// let ok = Result::<(),PalletAssetErr>::Ok(());
// env.write(ok.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call 'set_metadata'")
// })?;
// }
// DispatchResult::Err(e) => {
// error!("ERROR set_metadata");
// error!("{:#?}", e);
// let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
// env.write(err.encode().as_ref(), false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call 'set_metadata'")
// })?;
// }
// }
// }
//
// get asset metadata name
// 1113 => {
// error!("name metadata");
// use frame_support::traits::tokens::fungibles::metadata::Inspect;
//
// let mut env = env.buf_in_buf_out();
// let input: (OriginType, u32, [u8;32], [u8;32], u8) = env.read_as()?;
// let asset_id: u32 = env.read_as()?;
// use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::fungibles::metadata::Inspect;
// let name = Assets::name(asset_id);
//
// error!("asset_id : {}", asset_id);
// error!("name : {:#?}", name);
// let data = name.encode();
// env.write(&data[..], false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call balance")
// })?;
// }
//
// get asset metadata symbol
// 1114 => {
// error!("symbol metadata");
// use frame_support::traits::tokens::fungibles::metadata::Inspect;
//
// let mut env = env.buf_in_buf_out();
// let input: (OriginType, u32, [u8;32], [u8;32], u8) = env.read_as()?;
// let asset_id: u32 = env.read_as()?;
// use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::fungibles::metadata::Inspect;
// let symbol = Assets::symbol(asset_id);
//
// error!("asset_id : {}", asset_id);
// error!("symbol : {:#?}", symbol);
// let data = symbol.encode();
// env.write(&data[..], false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call balance")
// })?;
// }
//
// decimals
// 1115 => {
// error!("decimals metadata");
// use frame_support::traits::tokens::fungibles::metadata::Inspect;
// let mut env = env.buf_in_buf_out();
// let asset_id: u32 = env.read_as()?;
// use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::fungibles::metadata::Inspect;
// let decimals = Assets::decimals(asset_id);
//
// error!("asset_id : {}", asset_id);
// error!("decimals : {:#?}", decimals);
// let data = decimals.to_be_bytes();
// env.write(&data, false, None).map_err(|_| {
// DispatchError::Other("ChainExtension failed to call total_supply")
// })?;
// }
//
//
//
// _ => {
// error!("Called an unregistered `func_id`: {:}", func_id);
// return Err(DispatchError::Other("Unimplemented func_id"))
// }
// }
//
//
//
// let r = pallet_assets::Pallet::<Runtime>::total_supply(1);
//
//
//
// Ok(RetVal::Converging(0))
// }
//
// fn enabled() -> bool {
// true
// }
// }
//
