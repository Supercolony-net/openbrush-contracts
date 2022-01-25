pub use crate::{
    psp1155::*,
    traits::psp1155::extensions::burnable::*,
};
use brush::traits::{
    AccountId,
    Balance,
    InkStorage,
};
use ink_prelude::vec::Vec;

impl<T: PSP1155Internal + InkStorage> PSP1155Burnable for T {
    default fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._burn_from(from, ids_amounts)
    }
}