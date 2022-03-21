/// Extension of [`PSP34`] that adds enumerability of all the token ids in the contract as well
/// as all token ids owned by each account.
pub use crate::traits::psp34::*;
use brush::traits::AccountId;

#[brush::wrapper]
pub type PSP34EnumerableRef = dyn PSP34Enumerable + PSP34;

#[brush::trait_definition]
pub trait PSP34Enumerable: PSP34 {
    /// Returns a token `Id` owned by `owner` at a given `index` of its token list.
    /// Use along with {`balance_of`} to enumerate all of ``owner``'s tokens.
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error>;

    /// Returns a token `Id` at a given `index` of all the tokens stored by the contract.
    /// Use along with `total_supply` to enumerate all tokens.
    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error>;
}
