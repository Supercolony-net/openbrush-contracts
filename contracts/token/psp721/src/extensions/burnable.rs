/// Extension of [`PSP721`] that allows token holders to destroy their tokens
use crate::traits::*;

#[brush::trait_definition]
pub trait PSP721Burnable: IPSP721 {
    /// Destroys token with id equal to 'id'.
    ///
    /// See [`PSP721::_burn`].
    #[ink(message)]
    fn burn(&mut self, id: Id) {
        self._burn(id);
    }
}
