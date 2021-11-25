#![cfg_attr(not(feature = "std"), no_std)]

/// This will be a simple lending contract where users can:
///
/// 1. Lend tokens allowed by the smart contract.
/// The allowance and disallowance of tokens is done by the accounts which have an admin role
/// Upon lending, the user gets a PSP-22 token representing their share of the current liquidity pool
///
/// 2. Borrow tokens from the smart contract by depositing collateral tokens.
/// The tokens which can be deposited as a collateral are allowed and disallowed by the accounts with admin role
/// Upon borrowing user gets a PSP-721 token representing info about their loan (how much assets were borrowed,
/// when did they borrow, what asset was borrowed, what asset was used as a collateral, how much collateral assets
/// were deposited, the liquidation price of the loan and if it was liquidated or not)
///
/// 3. Repay their loan by depositing the borrowed amount of borrowed assets along with interest.
/// The contract determines how much a user needs to deposit and how much collateral they get back by an NFT token
/// which the user gets upon borrowing the assets. User is also able to repay a portion of the loan, but will only get
/// a portion of their collateral assets back, while the liquidation price will stay the same
///
/// 4. Swap their collateral tokens to repay the borrowed amount of borrowed assets with interest.
/// The contract will perform a swap of tokens for the borrowed token on a DEX, keep the borrowed amount + interest
/// and send the rest to the user
///
/// 5. Withdraw tokens deposited to the smart contract
/// User deposits their share tokens to the smart contract and the smart contract determines how much of the underlying
/// asset they get back
///
/// 6. Liquidate a loan
/// User can call a liquidation of a loan. If the price of collateral token of the loan is below or equal to the liquidation price,
/// the loan is then liquidated and the user performing the liquidation will get 1% of the liquidated assets
///
/// 7. Allow and disallow assets for lending
/// This can only be done by the accounts with the admin role
///
/// 8. Allow and disallow assets to be used as a collateral
/// This can only be done by the accounts with the admin role
///
/// 9. Pause the contract
/// Users with the admin role can pause the contract. If the contract is paused, no borrowing or lending can be performed
/// Users can still repay their loans, liquidate loans or withdraw their deposits
#[brush::contract]
pub mod lending {
    use access_control::traits::*;
    use brush::modifiers;
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use pausable::traits::*;

    /// Define the storage for PSP22 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, AccessControlStorage, PausableStorage)]
    pub struct Lending {
        #[AccessControlStorageField]
        access: AccessControlData,
        #[PausableStorageField]
        pause: PausableData,
    }

    const MANAGER: RoleType = ink_lang::selector_id!("MANAGER");

    impl AccessControl for Lending {}

    impl Pausable for Lending {}

    impl Lending {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            instance.grant_role(MANAGER, caller);
            instance
        }
    }
}
