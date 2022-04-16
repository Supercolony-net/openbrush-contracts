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
    type ErrorCode = RandomReadErr;

    /// Note: this gives the operation a corresponding `func_id` (1101 in this case),
    /// and the chain-side chain extension will get the `func_id` to do further operations.
    #[ink(extension = 1101, returns_result = false)]
    fn fetch_random(subject: [u8; 32]) -> [u8; 32];

    #[ink(extension = 1102, returns_result = false)]
    fn create(subject: PalletAssetRequest) -> [u8; 32];

    #[ink(extension = 1103, returns_result = false)]
    fn mint(subject: PalletAssetRequest) -> [u8; 32];

    #[ink(extension = 1104, returns_result = false)]
    fn burn(subject: PalletAssetRequest) -> [u8; 32];

    #[ink(extension = 1105, returns_result = false)]
    fn transfer(subject: PalletAssetRequest) -> [u8; 32];

    #[ink(extension = 1106, returns_result = false)]
    fn balance(subject: PalletAssetBalanceRequest) -> [u8; 16];
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
pub enum RandomReadErr {
    FailGetRandomSource,
}

impl ink_env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
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
    use super::RandomReadErr;
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
        #[ink(message)]
        pub fn update(&mut self, subject: [u8; 32]) -> Result<(), RandomReadErr> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random(subject)?;
            self.value = new_random;
            // Emit the `RandomUpdated` event when the random seed
            // is successfully fetched.
            self.env().emit_event(RandomUpdated { new: new_random });
            Ok(())
        }

        #[ink(message)]
        pub fn create_pallet_asset(&mut self, 
            origin_type: OriginType,
            asset_id : u32, 
            admin_address : [u8; 32], 
            min_balance : u128) -> Result<(), RandomReadErr> {
            // // create asset on-chain
            // let input = CreateAsset{origin_type, asset_id, admin_address, min_balance};
            // let new_random = self.env().extension().create(input)?;
            // is successfully minted.
            // self.env().emit_event();
            Ok(())
        }

        #[ink(message)]
        pub fn pallet_asset(&mut self, 
            asset_request: PalletAssetRequest, reqeust_type : RequestType) -> Result<(), RandomReadErr> {
            // mint asset on-chain
            let caller = self.env().caller();
            let r = caller.as_ref();
            let mut asset_request = asset_request;
            asset_request.target_address = *r;
            match reqeust_type{
                RequestType::Create => {
                    self.env().extension().create(asset_request)?;
                }
                RequestType::Mint => {
                    self.env().extension().mint(asset_request)?;
                }
                RequestType::Burn => {
                    self.env().extension().burn(asset_request)?;
                }
                RequestType::Transfer => {
                    self.env().extension().transfer(asset_request)?;
                }
            }
            // is successfully minted.
            // self.env().emit_event();
            Ok(())
        }

        #[ink(message)]
        pub fn balance_pallet_asset(&mut self, 
            asset_request: PalletAssetBalanceRequest) -> u128 {
            // mint asset on-chain
            let balance_bytes = self.env().extension().balance(asset_request).unwrap();
            let balance = u128::from_be_bytes(balance_bytes);
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
                /// `RandomReadErr`.
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

/// Contract extension for `FetchRandom`

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

// impl MaxEncodedLen for CreateInput{ }

impl ChainExtension<Runtime> for PalletAssetsExtention {
	

    fn call<E: Ext>(
        func_id: u32,
        mut env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId:
            UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
		// fn get_origin<E : Ext>(account : &<<E as Ext>::T as SysConfig>::AccountId) -> sp_core::crypto::AccountId32 
		// where <<E as Ext>::T as SysConfig>::AccountId: AsRef<[u8]>
		// {
		// 	let mut account_ref : &[u8] = account.as_ref();
		// 	let account_id = AccountId::decode(&mut account_ref).unwrap();
		// 	account_id
		// }

        match func_id {
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


				match create_result {
					DispatchResult::Ok(_) => trace!("OK"),
					DispatchResult::Err(e) => {
						error!("{:#?}", e);
						return Err(e);
					}
				}
				trace!("{:#?}", create_asset);
				let b = [1u8;32];
				//write buffer as responce for smart contract
                env.write(&b, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
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


				match mint_result {
					DispatchResult::Ok(_) => trace!("OK"),
					DispatchResult::Err(e) => {
						error!("{:#?}", e);
						return Err(e);
					}
				}
				trace!("{:#?}", mint_asset_request);
				let b = [1u8;32];
				//write buffer as responce for smart contract
                env.write(&b, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
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


				match burn_result {
					DispatchResult::Ok(_) => trace!("OK"),
					DispatchResult::Err(e) => {
						error!("{:#?}", e);
						return Err(e);
					}
				}
				trace!("{:#?}", burn_asset_request);
				let b = [1u8;32];
				//write buffer as responce for smart contract
                env.write(&b, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
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


				match tranfer_result {
					DispatchResult::Ok(_) => trace!("OK"),
					DispatchResult::Err(e) => {
						error!("{:#?}", e);
						return Err(e);
					}
				}
				trace!("{:#?}", transfer_asset_request);
				let b = [1u8;32];
				//write buffer as responce for smart contract
                env.write(&b, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
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

				trace!("OK! balance_of : {:#?}", balance_result);
				trace!("{:#?}", balance_asset_request);
				
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