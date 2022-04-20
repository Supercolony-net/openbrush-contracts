#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

/// This is an example of how an ink! contract may call the Substrate
/// runtime function `RandomnessCollectiveFlip::random_seed`. See the
/// file `runtime/chain-extension-example.rs` for that implementation.
///
/// Here we define the operations to interact with the Substrate runtime.
#[ink::chain_extension]
pub trait FetchRandom {
    type ErrorCode = PalletAssetErr;

    /// Note: this gives the operation a corresponding `func_id` (1101 in this case),
    /// and the chain-side chain extension will get the `func_id` to do further operations.
    #[ink(extension = 1101, returns_result = false)]
    fn fetch_random(subject: u8) -> Result<u8, PalletAssetErr>;

    #[ink(extension = 1102, returns_result = false)]
    fn create(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;

    #[ink(extension = 1103, returns_result = false)]
    fn mint(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;

    #[ink(extension = 1104, returns_result = false)]
    fn burn(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;

    #[ink(extension = 1105, returns_result = false)]
    fn transfer(subject: PalletAssetRequest) ->  Result<(), PalletAssetErr>;

    #[ink(extension = 1106, returns_result = false)]
    fn balance(subject: PalletAssetBalanceRequest) ->  u128;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RequestType{
	Create, 
	Mint,
	Burn,
	Transfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OriginType{
	Caller, 
	Address
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PalletAssetRequest{
	origin_type: OriginType,
	asset_id : u32, 
	target_address : [u8; 32], 
	amount : u128
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PalletAssetBalanceRequest{
	asset_id : u32, 
	address : [u8; 32], 
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
	//unknown error
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
	//unknown error
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
	//unknown error
    Unknown,
}

/*
fn get_error_code(dispatch_error : DispatchError) -> u32{
            match dispatch_error{
                DispatchError::Other(_) => 10,
                DispatchError::CannotLookup => 20,
                DispatchError::BadOrigin => 30,
                DispatchError::Module(e) => 40,
                DispatchError::ConsumerRemaining => 50,
                DispatchError::NoProviders => 60,
                DispatchError::TooManyConsumers => 70,
                DispatchError::Token(_) => 80,
                DispatchError::Arithmetic(e) => 90,
                _ => 100
            }
        }
*/

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchRandom;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod rand_extension {
    use super::PalletAssetErr;
    use crate::*;

    /// Defines the storage of our contract.
    ///
    /// Here we store the random seed fetched from the chain.
    #[ink(storage)]
    pub struct RandExtension {
        /// Stores a single `bool` value on the storage.
        value: [u8; 32],
    }

    #[ink(event)]
    pub struct RandomUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }

    impl RandExtension {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: [u8; 32]) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors may delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Seed a random value by passing some known argument `subject` to the runtime's
        /// random source. Then, update the current `value` stored in this contract with the
        /// new random value.
        // #[ink(message)]
        // pub fn update(&self, subject: u8) -> Result<u8, PalletAssetErr> {
        //     // Get the on-chain random seed
        //     let new_random = self.env().extension().fetch_random(subject);
        //     match new_random {
        //         Ok(_) => Ok(1),
        //         Err(_) => Err(PalletAssetErr::Other)
        //     }
            
        // }

        // #[ink(message)]
        // pub fn create_pallet_asset(&mut self, 
        //     origin_type: OriginType,
        //     asset_id : u32, 
        //     admin_address : [u8; 32], 
        //     min_balance : u128) -> Result<(), PalletAssetErr> {
        //     // // create asset on-chain
        //     // let input = CreateAsset{origin_type, asset_id, admin_address, min_balance};
        //     // let new_random = self.env().extension().create(input)?;
        //     // is successfully minted.
        //     // self.env().emit_event();
        //     Ok(())
        // }

        #[ink(message)]
        pub fn pallet_asset(&mut self, 
            asset_request: PalletAssetRequest, reqeust_type : RequestType) -> Result<(), PalletAssetErr> {
            // mint asset on-chain
            let caller = self.env().caller();
            let r = caller.as_ref();
            let mut asset_request = asset_request;
            asset_request.target_address = *r;
            match reqeust_type{
                RequestType::Create => {
                    self.env().extension().create(asset_request)?
                }
                RequestType::Mint => {
                    self.env().extension().mint(asset_request)?
                }
                RequestType::Burn => {
                    self.env().extension().burn(asset_request)?
                }
                RequestType::Transfer => {
                    self.env().extension().transfer(asset_request)?
                }
            }
        }

        

        #[ink(message)]
        pub fn balance_pallet_asset(&self, 
            asset_request: PalletAssetBalanceRequest) -> u128 {
            // mint asset on-chain
            let balance = self.env().extension().balance(asset_request).unwrap();
            // is successfully minted.
            // self.env().emit_event();
            balance
        }

        /// Simply returns the current value.
        #[ink(message)]
        pub fn get(&self) -> [u8; 32] {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let rand_extension = RandExtension::default();
            assert_eq!(rand_extension.get(), [0; 32]);
        }

        #[ink::test]
        fn chain_extension_works() {
            // given
            struct MockedExtension;
            impl ink_env::test::ChainExtension for MockedExtension {
                /// The static function id of the chain extension.
                fn func_id(&self) -> u32 {
                    1101
                }

                /// The chain extension is called with the given input.
                ///
                /// Returns an error code and may fill the `output` buffer with a
                /// SCALE encoded result. The error code is taken from the
                /// `ink_env::chain_extension::FromStatusCode` implementation for
                /// `PalletAssetErr`.
                fn call(&mut self, _input: &[u8], output: &mut Vec<u8>) -> u32 {
                    let ret: [u8; 32] = [1; 32];
                    scale::Encode::encode_to(&ret, output);
                    0
                }
            }
            ink_env::test::register_chain_extension(MockedExtension);
            let mut rand_extension = RandExtension::default();
            assert_eq!(rand_extension.get(), [0; 32]);

            // when
            rand_extension.update([0_u8; 32]).expect("update must work");

            // then
            assert_eq!(rand_extension.get(), [1; 32]);
        }
    }
}

/*

use frame_support::log::{
    error,
    trace,
};

use pallet_contracts::chain_extension::{
    ChainExtension,
    Environment,
    Ext,
    InitState,
    RetVal,
    SysConfig,
    UncheckedFrom,
};

use sp_runtime::DispatchError;
use sp_runtime::TokenError;
use sp_runtime::ArithmeticError;

use sp_runtime::MultiAddress;
pub struct PalletAssetsExtention;

// struct Origin{}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode,  MaxEncodedLen)]
enum OriginType{
	Caller, 
	Address
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
struct PalletAssetRequest{
	origin_type: OriginType,
	asset_id : u32, 
	target_address : [u8; 32], 
	amount : u128
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
struct PalletAssetBalanceRequest{
	asset_id : u32, 
	address : [u8; 32], 
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
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
	//unknown error
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
pub enum PalletAssetArithmeticErr {
	/// Underflow.
	Underflow,
	/// Overflow.
	Overflow,
	/// Division by zero.
	DivisionByZero,
	//unknown error
    Unknown,

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
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
	//unknown error
    Unknown,
}

impl From<DispatchError> for PalletAssetErr {
    fn from(e: DispatchError) -> Self {
        match e{
			DispatchError::Other(_) => PalletAssetErr::Other,
			DispatchError::CannotLookup => PalletAssetErr::CannotLookup,
			DispatchError::BadOrigin => PalletAssetErr::BadOrigin,
			DispatchError::Module(_) => PalletAssetErr::Module,
			DispatchError::ConsumerRemaining => PalletAssetErr::ConsumerRemaining,
			DispatchError::NoProviders => PalletAssetErr::NoProviders,
			DispatchError::TooManyConsumers => PalletAssetErr::TooManyConsumers,
			DispatchError::Token(token_err) => PalletAssetErr::Token(PalletAssetTokenErr::from(token_err)),
			DispatchError::Arithmetic(arithmetic_error) => PalletAssetErr::Arithmetic(PalletAssetArithmeticErr::from(arithmetic_error)),
			_ => PalletAssetErr::Unknown,
		}
    }
}

impl From<ArithmeticError> for PalletAssetArithmeticErr {
    fn from(e: ArithmeticError) -> Self {
        match e{
			ArithmeticError::Underflow => PalletAssetArithmeticErr::Underflow,
			ArithmeticError::Overflow => PalletAssetArithmeticErr::Overflow,
			ArithmeticError::DivisionByZero => PalletAssetArithmeticErr::DivisionByZero,
			_ => PalletAssetArithmeticErr::Unknown,
		}
    }
}

impl From<TokenError> for PalletAssetTokenErr {
    fn from(e: TokenError) -> Self {
        match e{
			TokenError::NoFunds => PalletAssetTokenErr::NoFunds,
			TokenError::WouldDie => PalletAssetTokenErr::WouldDie,
			TokenError::BelowMinimum => PalletAssetTokenErr::BelowMinimum,
			TokenError::CannotCreate => PalletAssetTokenErr::CannotCreate,
			TokenError::UnknownAsset => PalletAssetTokenErr::UnknownAsset,
			TokenError::Frozen => PalletAssetTokenErr::Frozen,
			TokenError::Unsupported => PalletAssetTokenErr::Unsupported,
			_ => PalletAssetTokenErr::Unknown,
		}
    }
}




impl ChainExtension<Runtime> for PalletAssetsExtention {

    fn call<E: Ext>(
        func_id: u32,
        mut env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId:
            UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        match func_id {

			//create

			1101 => {
				let ext = env.ext();
				let address : &<<E as Ext>::T as SysConfig>::AccountId = ext.address();
				let caller = ext.caller();
				let mut caller_ref = caller.as_ref();
				let caller_accountId = AccountId::decode(&mut caller_ref).unwrap();

				use frame_support::dispatch::DispatchError;
				use frame_support::dispatch::DispatchResult;
				
				let mut address_ref = caller.as_ref();
				let address_account = AccountId::decode(&mut address_ref).unwrap();
				let create_result = pallet_assets::Pallet::<Runtime>::
				create(Origin::signed(caller_accountId.clone()), 1, MultiAddress::Id(address_account.clone()), 1);
				match create_result {
					DispatchResult::Ok(_) => error!("OK"),
					DispatchResult::Err(e) => error!("{:#?}", e)
				}
				//enum (caller, address_account)
				//asset id
				let mint_result = 
				pallet_assets::Pallet::<Runtime>::
				mint(Origin::signed(caller_accountId), 1, MultiAddress::Id(address_account), 10);
				match mint_result {
					DispatchResult::Ok(_) => error!("OK"),
					DispatchResult::Err(e) => error!("{:#?}", e)
				}

				let r = pallet_assets::Pallet::<Runtime>::total_supply(1);
				error!("total_supply: {:}", r);
				//return Err(DispatchError::Other("Unimplemented func_id"))
                let mut env = env.buf_in_buf_out();
                let arg: [u8; 32] = env.read_as()?;
                // let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
                // let random_slice = random_seed.encode();
                // trace!(
                //     target: "runtime",
                //     "[ChainExtension]|call|func_id:{:}",
                //     func_id
                // );
                env.write(&arg, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
            }

			1100 => {
				let ext = env.ext();
				let mut env = env.buf_in_buf_out();
				error!("ERROR test");
				let err = Result::<u8,PalletAssetErr>::Err(PalletAssetErr::Other);
				env.write(err.encode().as_ref(), false, None).map_err(|_| {
					DispatchError::Other("ChainExtension failed to call test")
				})?;
			}

			//create
			1102 => {
				let ext = env.ext();
				let address  = ext.address();
				let caller = ext.caller();
				let mut caller_ref = caller.as_ref();
				let mut address_ref = address.as_ref();
				let caller_account = AccountId::decode(&mut caller_ref).unwrap();
				let address_account = AccountId::decode(&mut address_ref).unwrap();
				

				use frame_support::dispatch::DispatchResult;

                let mut env = env.buf_in_buf_out();
                let create_asset: PalletAssetRequest = env.read_as()?;

				let origin_address = match create_asset.origin_type {
					OriginType::Caller => {
						caller_account
					},
					OriginType::Address => {
						address_account
					},
				};

				let mut vec = &create_asset.target_address.to_vec()[..];
				let admin_address = AccountId::decode(&mut vec).unwrap();
				let create_result = pallet_assets::Pallet::<Runtime>::
				create(Origin::signed(origin_address), 
				create_asset.asset_id, 
				MultiAddress::Id(admin_address), 
				create_asset.amount);

				error!("{:#?}", create_asset);
				error!("{:#?}", create_result);
				match create_result {
					DispatchResult::Ok(_) => {
						error!("OK create");
						let err = Result::<(),PalletAssetErr>::Ok(());
						env.write(err.encode().as_ref(), false, None).map_err(|_| {
							DispatchError::Other("ChainExtension failed to call create")
						})?;
					}
					DispatchResult::Err(e) => {
						error!("ERROR create");
						error!("{:#?}", e);
						let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
						env.write(err.encode().as_ref(), false, None).map_err(|_| {
							DispatchError::Other("ChainExtension failed to call create")
						})?;
					}
				}
				
            }

			//mint
			1103 => {
				let ext = env.ext();
				let address  = ext.address();
				let caller = ext.caller();
				let mut caller_ref = caller.as_ref();
				let mut address_ref = address.as_ref();
				let caller_account = AccountId::decode(&mut caller_ref).unwrap();
				let address_account = AccountId::decode(&mut address_ref).unwrap();
				

				use frame_support::dispatch::DispatchResult;

                let mut env = env.buf_in_buf_out();
                let mint_asset_request: PalletAssetRequest = env.read_as()?;

				let origin_address = match mint_asset_request.origin_type {
					OriginType::Caller => {
						caller_account
					},
					OriginType::Address => {
						address_account
					},
				};

				let mut vec = &mint_asset_request.target_address.to_vec()[..];
				let beneficiary_address = AccountId::decode(&mut vec).unwrap();
				let mint_result = pallet_assets::Pallet::<Runtime>::
				mint(Origin::signed(origin_address),
				mint_asset_request.asset_id, 
				MultiAddress::Id(beneficiary_address), 
				mint_asset_request.amount);

				error!("{:#?}", mint_asset_request);
				error!("{:#?}", mint_result);
				match mint_result {
					DispatchResult::Ok(_) => {
						error!("OK mint")
					},
					DispatchResult::Err(e) => {
						error!("ERROR mint");
						error!("{:#?}", e);
						let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
						env.write(err.encode().as_ref(), false, None).map_err(|_| {
							DispatchError::Other("ChainExtension failed to call mint")
						})?;
					}
				}
            }

			//burn
			1104 => {
				let ext = env.ext();
				let address  = ext.address();
				let caller = ext.caller();
				let mut caller_ref = caller.as_ref();
				let mut address_ref = address.as_ref();
				let caller_account = AccountId::decode(&mut caller_ref).unwrap();
				let address_account = AccountId::decode(&mut address_ref).unwrap();
				

				use frame_support::dispatch::DispatchResult;

                let mut env = env.buf_in_buf_out();
                let burn_asset_request: PalletAssetRequest = env.read_as()?;

				let origin_address = match burn_asset_request.origin_type {
					OriginType::Caller => {
						caller_account
					},
					OriginType::Address => {
						address_account
					},
				};

				let mut vec = &burn_asset_request.target_address.to_vec()[..];
				let who_address = AccountId::decode(&mut vec).unwrap();
				let burn_result = pallet_assets::Pallet::<Runtime>::
				burn(Origin::signed(origin_address),
				burn_asset_request.asset_id, 
				MultiAddress::Id(who_address), 
				burn_asset_request.amount);

				error!("{:#?}", burn_asset_request);
				error!("{:#?}", burn_result);
				match burn_result {
					DispatchResult::Ok(_) => {
						error!("OK burn")
					}
					DispatchResult::Err(e) => {
						error!("ERROR burn");
						error!("{:#?}", e);
						let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
						env.write(err.encode().as_ref(), false, None).map_err(|_| {
							DispatchError::Other("ChainExtension failed to call burn")
						})?;
					}
				}
            }

			//transfer
			1105 => {
				let ext = env.ext();
				let address  = ext.address();
				let caller = ext.caller();
				let mut caller_ref = caller.as_ref();
				let mut address_ref = address.as_ref();
				let caller_account = AccountId::decode(&mut caller_ref).unwrap();
				let address_account = AccountId::decode(&mut address_ref).unwrap();
				

				use frame_support::dispatch::DispatchResult;

                let mut env = env.buf_in_buf_out();
                let transfer_asset_request: PalletAssetRequest = env.read_as()?;

				let origin_address = match transfer_asset_request.origin_type {
					OriginType::Caller => {
						caller_account
					},
					OriginType::Address => {
						address_account
					},
				};

				let mut vec = &transfer_asset_request.target_address.to_vec()[..];
				let target_address = AccountId::decode(&mut vec).unwrap();
				let tranfer_result = pallet_assets::Pallet::<Runtime>::
				transfer(Origin::signed(origin_address),
				transfer_asset_request.asset_id, 
				MultiAddress::Id(target_address), 
				transfer_asset_request.amount);

				trace!("{:#?}", transfer_asset_request);
				trace!("{:#?}", tranfer_result);
				match tranfer_result {
					DispatchResult::Ok(_) => {
						error!("OK transfer")
						//write buffer as responce for smart contract
						// let b = [1u8;32];
						// env.write(&b, false, None).map_err(|_| {
						// 	DispatchError::Other("ChainExtension failed to call random")
						// })?;
					}
					DispatchResult::Err(e) => {
						error!("ERROR transfer");
						error!("{:#?}", e);
						let err = Result::<(),PalletAssetErr>::Err(PalletAssetErr::from(e));
						env.write(err.encode().as_ref(), false, None).map_err(|_| {
							DispatchError::Other("ChainExtension failed to call burn")
						})?;
					}
				}
            }
			
			//balance
			1106 => {
				let ext = env.ext();
				let address  = ext.address();
				let caller = ext.caller();
				let mut caller_ref = caller.as_ref();
				let mut address_ref = address.as_ref();
				let caller_account = AccountId::decode(&mut caller_ref).unwrap();
				let address_account = AccountId::decode(&mut address_ref).unwrap();
				

				use frame_support::dispatch::DispatchResult;

                let mut env = env.buf_in_buf_out();
                let balance_asset_request: PalletAssetBalanceRequest = env.read_as()?;

				
				let mut vec = &balance_asset_request.address.to_vec()[..];
				let balance_of_address = AccountId::decode(&mut vec).unwrap();
				let balance_result : Balance = pallet_assets::Pallet::<Runtime>::
				balance(balance_asset_request.asset_id,balance_of_address);

				error!("OK! balance_of : {:#?}", balance_result);
				error!("{:#?}", balance_asset_request);
				
				let b = balance_result.to_be_bytes();
				//write buffer as responce for smart contract
                env.write(&b, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
            }

			
            _ => {
                error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        }

		

        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}

*/