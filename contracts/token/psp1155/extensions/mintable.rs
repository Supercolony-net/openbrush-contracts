pub use crate::{
    psp1155::*,
    traits::psp1155::extensions::mintable::*,
};
use brush::traits::{
    AccountId,
    Balance,
    InkStorage,
};
use ink_prelude::vec::Vec;

impl<T: PSP1155Internal + InkStorage> PSP1155Mintable for T {
    default fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._mint_to(to, ids_amounts)
    }
}
