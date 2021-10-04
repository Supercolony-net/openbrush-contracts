/// Extension of [`PSP22`] that allows token holders to destroy both their own
/// tokens and those that they have an allowance for, in a way that can be
/// recognized off-chain (via event analysis).
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::trait_definition]
pub trait PSP22Burnable: PSP22 {
    /// Destroys `amount` tokens from the caller.
    ///
    /// See [`PSP22::_burn`].
    #[ink(message)]
    fn burn(&mut self, amount: Balance) {
        self._burn(Self::env().caller(), amount);
    }

    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22::_burn`] and [`PSP22::allowance`].
    ///
    /// Requirements:
    ///
    /// - the caller must have allowance for ``accounts``'s tokens of at least
    /// `amount`.
    /// # Errors
    ///
    /// Panics with `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    #[ink(message)]
    fn burn_from(&mut self, account: AccountId, amount: Balance) {
        let current_allowance = *self.get().allowances.get(&(account, Self::env().caller()));

        assert!(
            current_allowance
                .and_then(|allowance| Some(allowance >= &amount))
                .unwrap_or(false),
            "PSP22: burn amount exceeds allowance"
        );
        self.approve(
            account,
            current_allowance
                .and_then(|allowance| Some(allowance - &amount))
                .unwrap(),
        );

        self._burn(account, amount);
    }
}
