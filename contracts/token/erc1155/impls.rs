use crate::traits::{Erc1155Error, Id};
#[cfg(not(test))]
use ink_env::{
    call::{build_call, utils::ReturnType, ExecutionInput, Selector},
    DefaultEnvironment, Error as Env_error,
};
use ink_prelude::{vec::Vec, string::String, vec};
use ink_storage::{
    collections::HashMap as StorageHashMap,
};
use utils::{
    traits::{InkStorage, AccountId, Balance},
    define_getters,
};

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Erc1155MetadataURIStorage: InkStorage {
    define_getters!(_uri, _uri_mut, Option<String>);
}

pub trait Erc1155MetadataURI: Erc1155MetadataURIStorage {
    fn new(uri: Option<String>) -> Self {
        let mut instance = Self::_empty();
        *instance._uri_mut() = uri;
        instance
    }

    // This method is available from Erc1155MetadataURIStorage
    // fn _uri(&self, _id: Id) -> Option<String>;
}

pub trait Erc1155Storage: InkStorage {
    define_getters!(_balances, _balances_mut, StorageHashMap<(Id, AccountId), Balance>);

    define_getters!(_operator_approval, _operator_approval_mut, StorageHashMap<(AccountId, AccountId), bool>);
}

pub trait Erc1155Internal: Erc1155Storage {
    fn _emit_transfer_single_event(&self,
                                   _operator: AccountId, _from: AccountId,
                                   _to: AccountId, _id: Id, _amount: Balance);

    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool);

    fn _emit_transfer_batch_event(&self,
                                  _operator: AccountId, _from: AccountId,
                                  _to: AccountId, _ids: Vec<Id>, _amounts: Vec<Balance>);

    #[inline]
    fn _transfer_guard(&self, from: AccountId, to: AccountId) -> Result<(), Erc1155Error> {
        if to == ZERO_ADDRESS.into() {
            return Err(Erc1155Error::TransferToZeroAddress);
        }

        let operator = Self::env().caller();

        if (from != operator) && (!self._is_approved_for_all(from, operator)) {
            return Err(Erc1155Error::ApproveRequired);
        }

        Ok(())
    }

    #[inline]
    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) -> Result<(), Erc1155Error> {
        self._decrease_sender_balance(from, id, amount)?;
        self._increase_receiver_balance(to, id, amount)?;

        Ok(())
    }

    #[inline]
    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self._balances().get(&(id, owner)).cloned().unwrap_or(0)
    }
    #[inline]
    fn _is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
        self._operator_approval().get(&(_account, _operator)).cloned().unwrap_or(false)
    }

    #[inline]
    fn _mint(&mut self, to: AccountId, id: Id, amount: Balance) -> Result<(), Erc1155Error> {
        let operator = Self::env().caller();

        if to == ZERO_ADDRESS.into() {
            return Err(Erc1155Error::TransferToZeroAddress);
        }

        self._before_token_transfer(&vec![id])?;
        self._increase_receiver_balance(to, id, amount)?;

        self._do_safe_transfer_acceptance_check(
            operator,
            ZERO_ADDRESS.into(),
            to,
            id,
            amount,
            Vec::new(),
        )?;

        self._emit_transfer_single_event(
            operator, ZERO_ADDRESS.into(), to, id, amount);
        Ok(())
    }

    #[inline]
    fn _burn(&mut self, from: AccountId, id: Id, amount: Balance) -> Result<(), Erc1155Error> {
        if from == ZERO_ADDRESS.into() {
            return Err(Erc1155Error::TransferToZeroAddress);
        }

        self._before_token_transfer(&vec![id])?;
        self._decrease_sender_balance(from, id, amount)?;

        self._emit_transfer_single_event(
            Self::env().caller(), from, ZERO_ADDRESS.into(), id, amount);
        Ok(())
    }

    #[inline]
    fn _increase_receiver_balance(
        &mut self,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) -> Result<(), Erc1155Error> {
        let to_balance = self._balances_mut().entry((id, to)).or_insert(0);
        match to_balance.checked_add(amount) {
            Some(new_to_balance) => *to_balance = new_to_balance,
            _ => return Err(Erc1155Error::MaxBalance),
        }
        Ok(())
    }

    #[inline]
    fn _decrease_sender_balance(
        &mut self,
        from: AccountId,
        id: Id,
        amount: Balance,
    ) -> Result<(), Erc1155Error> {
        match self
            ._balances()
            .get(&(id, from))
            .map(|old_from_balance| old_from_balance.checked_sub(amount))
        {
            Some(Some(new_from_balance)) => self._balances_mut().insert((id, from), new_from_balance),
            _ => return Err(Erc1155Error::InsufficientBalance),
        };
        Ok(())
    }

    #[inline]
    fn _before_token_transfer(&self, _ids: &Vec<Id>) -> Result<(), Erc1155Error> {
        Ok(())
    }

    #[cfg(test)]
    #[inline]
    fn _do_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), Erc1155Error> {
        Ok(())
    }

    #[cfg(not(test))]
    #[inline]
    fn _do_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), Erc1155Error> {
        match build_call::<DefaultEnvironment>()
            .callee(_to)
            .exec_input(
                // ::ink_lang_ir::Selector::new("IErc1155Receiver::on_erc1155_received".as_ref()).as_bytes()
                ExecutionInput::new(Selector::new([0xc4, 0xce, 0x40, 0x79]))
                    .push_arg(_operator)
                    .push_arg(_from)
                    .push_arg(_id)
                    .push_arg(_amount)
                    .push_arg(_data),
            )
            .returns::<ReturnType<Result<(), Erc1155Error>>>()
            .fire()
        {
            Ok(result) => match result {
                Ok(_) => Ok(()),
                _ => Err(Erc1155Error::CallFailed),
            },
            Err(e) => match e {
                Env_error::NotCallable => Ok(()),
                _ => Err(Erc1155Error::CallFailed),
            },
        }
    }

    #[cfg(test)]
    #[inline]
    fn _do_batch_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) -> Result<(), Erc1155Error> {
        Ok(())
    }

    #[cfg(not(test))]
    #[inline]
    fn _do_batch_safe_transfer_acceptance_check(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) -> Result<(), Erc1155Error> {
        match build_call::<DefaultEnvironment>()
            .callee(_to)
            .exec_input(
                // ::ink_lang_ir::Selector::new("IErc1155Receiver::on_erc1155_batch_received".as_ref()).as_bytes()
                ExecutionInput::new(Selector::new([0xcc, 0xfa, 0x60, 0x0e]))
                    .push_arg(_operator)
                    .push_arg(_from)
                    .push_arg(_ids)
                    .push_arg(_amounts)
                    .push_arg(_data),
            )
            .returns::<ReturnType<Result<(), Erc1155Error>>>()
            .fire()
        {
            Ok(result) => match result {
                Ok(_) => Ok(()),
                _ => Err(Erc1155Error::CallFailed),
            },
            Err(e) => match e {
                Env_error::NotCallable => Ok(()),
                _ => Err(Erc1155Error::CallFailed),
            },
        }
    }
}

pub trait Erc1155: Erc1155Internal {
    fn _balance_of(&self, _account: AccountId, _id: Id) -> Balance {
        self._balance_of_or_zero(_account, _id)
    }

    fn _balance_of_batch(
        &self,
        _accounts: Vec<AccountId>,
        _ids: Vec<Id>,
    ) -> Result<Vec<Balance>, Erc1155Error> {
        if _accounts.len() != _ids.len() {
            return Err(Erc1155Error::InputLengthMismatch);
        }

        let values: Vec<Balance> = _accounts
            .iter()
            .zip(_ids.iter())
            .map(|(account, id)| self._balance_of_or_zero(account.clone(), id.clone()))
            .collect();
        Ok(values)
    }

    fn _set_approval_for_all(&mut self, _operator: AccountId, _approved: bool) -> Result<(), Erc1155Error> {
        let caller = Self::env().caller();
        if caller == _operator {
            return Err(Erc1155Error::SelfApproval);
        }
        *self
            ._operator_approval_mut()
            .entry((Self::env().caller(), _operator))
            .or_insert(false) = _approved;

        self._emit_approval_for_all_event(caller, _operator, _approved);
        Ok(())
    }

    // It is implemented in Erc1155Internal
    // fn _is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool;

    fn _safe_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), Erc1155Error> {
        self._transfer_guard(_from, _to)?;
        self._before_token_transfer(&vec![_id])?;
        self._transfer_from(_from, _to, _id, _amount)?;

        self._do_safe_transfer_acceptance_check(
            Self::env().caller(),
            _from,
            _to,
            _id,
            _amount,
            _data
        )?;

        self._emit_transfer_single_event(
            Self::env().caller(), _from, _to, _id, _amount);
        Ok(())
    }

    fn _safe_batch_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) -> Result<(), Erc1155Error> {
        if _ids.len() != _amounts.len() {
            return Err(Erc1155Error::InputLengthMismatch);
        }
        self._transfer_guard(_from, _to)?;
        self._before_token_transfer(&_ids)?;

        for (id, value) in _ids.iter().zip(_amounts.iter()) {
            self._transfer_from(_from, _to, id.clone(), value.clone())?;
        }

        self._do_batch_safe_transfer_acceptance_check(
            Self::env().caller(),
            _from,
            _to,
            _ids.clone(),
            _amounts.clone(),
            _data,
        )?;

        self._emit_transfer_batch_event(
            Self::env().caller(), _from, _to, _ids, _amounts);
        Ok(())
    }
}

#[cfg(test)]
#[ink_lang::contract]
mod tests {
    use super::*;
    use ink_lang as ink;
    use utils::{
        traits::{InkStorage},
        iml_getters, assert_ok, assert_err,
    };
    use ink::{Env, EmitEvent};
    use crate::traits::{ IErc1155 };

    #[ink(event)]
    pub struct TransferSingle {
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        id: Id,
        value: Balance,
    }

    #[ink(event)]
    pub struct TransferBatch {
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        ids: Vec<Id>,
        values: Vec<Balance>,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc1155Struct {
        balances: StorageHashMap<(Id, AccountId), Balance>,
        operator_approval: StorageHashMap<(AccountId, AccountId), bool>,
    }

    impl InkStorage for Erc1155Struct {}
    impl Erc1155Storage for Erc1155Struct {
        iml_getters!(balances, _balances, _balances_mut, StorageHashMap<(Id, AccountId), Balance>);
        iml_getters!(operator_approval, _operator_approval, _operator_approval_mut, StorageHashMap<(AccountId, AccountId), bool>);
    }
    impl Erc1155Internal for Erc1155Struct {
        fn _emit_transfer_single_event(&self,
                _operator: AccountId, _from: AccountId, _to: AccountId, _id: Id, _amount: Balance) {
            self.env().emit_event(TransferSingle {
                operator: _operator,
                from: _from,
                to: _to,
                id: _id,
                value: _amount,
            });
        }

        fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner: _owner,
                operator: _operator,
                approved: _approved,
            });
        }

        fn _emit_transfer_batch_event(&self,
                _operator: AccountId, _from: AccountId, _to: AccountId, _ids: Vec<Id>, _amounts: Vec<Balance>) {
            self.env().emit_event(TransferBatch {
                operator: _operator,
                from: _from,
                to: _to,
                ids: _ids,
                values: _amounts,
            });
        }
    }
    impl Erc1155 for Erc1155Struct {}

    impl IErc1155 for Erc1155Struct {
        #[ink(message)]
        fn balance_of(&self, _account: AccountId, _id: Id) -> Balance {
            self._balance_of(_account, _id)
        }

        #[ink(message)]
        fn balance_of_batch(&self, _owners: Vec<AccountId>, _ids: Vec<Id>) -> Result<Vec<Balance>, Erc1155Error> {
            self._balance_of_batch(_owners, _ids)
        }

        #[ink(message)]
        fn set_approval_for_all(&mut self, _operator: AccountId, _approved: bool) -> Result<(), Erc1155Error> {
            self._set_approval_for_all(_operator, _approved)
        }

        #[ink(message)]
        fn is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
            self._is_approved_for_all(_account, _operator)
        }

        #[ink(message)]
        fn safe_transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _amount: Balance,
            _data: Vec<u8>,
        ) -> Result<(), Erc1155Error> {
            self._safe_transfer_from(_from, _to, _id, _amount, _data)
        }

        #[ink(message)]
        fn safe_batch_transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _ids: Vec<Id>,
            _amounts: Vec<Balance>,
            _data: Vec<u8>,
        ) -> Result<(), Erc1155Error> {
            self._safe_batch_transfer_from(_from, _to, _ids, _amounts, _data)
        }
    }

    impl Erc1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::_empty()
        }
    }

    type Event = <Erc1155Struct as ::ink_lang::BaseEvent>::Type;

    #[ink::test]
    fn balance_of() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        // Token 1 does not exists.
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        // mint some token 1
        nft._mint(accounts.alice, token_id, 1).unwrap();
        assert_eq!(nft.balance_of(accounts.alice, token_id), mint_amount);

        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id,
            mint_amount,
        );
        assert_eq!(ink_env::test::recorded_events().count(), 1);
    }

    #[ink::test]
    fn balance_of_batch() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        // Token 1 does not exists.
        assert_eq!(
            nft
                .balance_of_batch(
                    vec![accounts.alice, accounts.alice],
                    vec![token_id_1, token_id_2]
                )
                .unwrap(),
            vec![0, 0]
        );
        // mint some token 1
        assert_ok!(nft._mint(accounts.alice, token_id_1, token_1_amount));
        assert_eq!(
            nft
                .balance_of_batch(
                    vec![accounts.alice, accounts.alice],
                    vec![token_id_1, token_id_2]
                )
                .unwrap(),
            vec![token_1_amount, 0]
        );

        // mint some token 2
        assert_ok!(nft._mint(accounts.bob, token_id_2, token_2_amount));
        assert_eq!(
            nft
                .balance_of_batch(
                    vec![accounts.alice, accounts.bob],
                    vec![token_id_1, token_id_2]
                )
                .unwrap(),
            vec![token_1_amount, token_2_amount]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.bob,
            token_id_2,
            token_2_amount,
        );
        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn set_approval_for_all() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        // no approvall exists yet
        assert!(!nft.is_approved_for_all(accounts.alice, accounts.bob),);
        // set approval
        assert_ok!(nft.set_approval_for_all(accounts.bob, true));
        // approval exists
        assert!(nft.is_approved_for_all(accounts.alice, accounts.bob));
        // remove approval
        assert_ok!(nft.set_approval_for_all(accounts.bob, false));
        // no approvall exists
        assert!(!nft.is_approved_for_all(accounts.alice, accounts.bob));

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, false);

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_single() {
        let token_id = [1; 32];
        let transfer_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.alice, token_id, transfer_amount));
        assert_ok!(nft.safe_transfer_from(
            accounts.alice,
            accounts.bob,
            token_id,
            transfer_amount,
            [].to_vec()
        ));
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), transfer_amount);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id,
            transfer_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            accounts.alice,
            accounts.bob,
            token_id,
            transfer_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_batch() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.alice, token_id_1, token_1_amount));
        assert_ok!(nft._mint(accounts.alice, token_id_2, token_2_amount));
        assert_ok!(nft.safe_batch_transfer_from(
            accounts.alice,
            accounts.bob,
            ids.clone(),
            amounts.clone(),
            [].to_vec()
        ),);

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.bob, accounts.bob], ids.clone())
                .unwrap(),
            amounts.clone()
        );
        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.alice, accounts.alice], ids.clone())
                .unwrap(),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_2,
            token_2_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_batch_event(
            emmited_event,
            accounts.alice,
            accounts.alice,
            accounts.bob,
            &ids,
            &amounts,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn transfer_from_single_insufficient_balance() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let transfer_amount = 2;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.alice, token_id, mint_amount));
        assert_err!(
            nft.safe_transfer_from(
                accounts.alice,
                accounts.bob,
                token_id,
                transfer_amount,
                [].to_vec()
            ),
            Erc1155Error::InsufficientBalance
        );
        assert_eq!(nft.balance_of(accounts.alice, token_id), mint_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id), 0);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id,
            mint_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 1);
    }

    #[ink::test]
    fn transfer_from_single_no_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.bob, token_id, mint_amount));
        assert_err!(
            nft.safe_transfer_from(
                accounts.bob,
                accounts.alice,
                token_id,
                mint_amount,
                [].to_vec(),
            ),
            Erc1155Error::ApproveRequired
        );

        assert_eq!(nft.balance_of(accounts.bob, token_id), mint_amount);
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.bob,
            token_id,
            mint_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 1);
    }

    #[ink::test]
    fn transfer_from_single_with_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.alice, token_id, mint_amount));
        assert_ok!(nft.set_approval_for_all(accounts.bob, true));

        // CHANGE CALLEE MANUALLY
        // Get contract address.
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
        // Create call.
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );
        assert_ok!(nft.safe_transfer_from(
            accounts.alice,
            accounts.bob,
            token_id,
            mint_amount,
            [].to_vec()
        ));

        assert_eq!(nft.balance_of(accounts.bob, token_id), mint_amount);
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id,
            mint_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.bob,
            accounts.alice,
            accounts.bob,
            token_id,
            mint_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn transfer_from_batch_insufficient_balance() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let wrong_amounts = vec![2, 21]; //TODO currently transaction is not reverted on error. Fix test case in future, when 1st amount is correct
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.alice, token_id_1, token_1_amount));
        assert_ok!(nft._mint(accounts.alice, token_id_2, token_2_amount));
        assert_err!(
            nft.safe_batch_transfer_from(
                accounts.alice,
                accounts.bob,
                ids.clone(),
                wrong_amounts.clone(),
                [].to_vec()
            ),
            Erc1155Error::InsufficientBalance
        );

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.alice, accounts.alice], ids.clone())
                .unwrap(),
            amounts.clone()
        );

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.bob, accounts.bob], ids.clone())
                .unwrap(),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_2,
            token_2_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_batch_no_approve() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.bob, token_id_1, token_1_amount));
        assert_ok!(nft._mint(accounts.bob, token_id_2, token_2_amount));

        assert_err!(
            nft.safe_batch_transfer_from(
                accounts.bob,
                accounts.alice,
                ids.clone(),
                amounts.clone(),
                [].to_vec()
            ),
            Erc1155Error::ApproveRequired
        );

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.bob, accounts.bob], ids.clone())
                .unwrap(),
            amounts.clone()
        );

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.alice, accounts.alice], ids.clone())
                .unwrap(),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.bob,
            token_id_1,
            token_1_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.bob,
            token_id_2,
            token_2_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_batch_with_approve() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc1155Struct::new();
        assert_ok!(nft._mint(accounts.alice, token_id_1, token_1_amount));
        assert_ok!(nft._mint(accounts.alice, token_id_2, token_2_amount));
        assert_ok!(nft.set_approval_for_all(accounts.bob, true));

        // CHANGE CALLEE MANUALLY
        // Get contract address.
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
        // Create call.
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );
        assert_ok!(nft.safe_batch_transfer_from(
            accounts.alice,
            accounts.bob,
            ids.clone(),
            amounts.clone(),
            [].to_vec()
        ));

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.bob, accounts.bob], ids.clone())
                .unwrap(),
            amounts.clone()
        );

        assert_eq!(
            nft
                .balance_of_batch(vec![accounts.alice, accounts.alice], ids.clone())
                .unwrap(),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_2,
            token_2_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_batch_event(
            emmited_event,
            accounts.bob,
            accounts.alice,
            accounts.bob,
            &ids,
            &amounts,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 4);
    }

    fn assert_transfer_event(
        event: ink_env::test::EmittedEvent,
        expected_operator: AccountId,
        expected_from: AccountId,
        expected_to: AccountId,
        expected_token_id: Id,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::TransferSingle(TransferSingle {
            operator,
            from,
            to,
            id,
            value,
        }) = decoded_event
        {
            assert_eq!(
                operator, expected_operator,
                "encountered invalid TransferSingle.operator"
            );
            assert_eq!(
                from, expected_from,
                "encountered invalid TransferSingle.from"
            );
            assert_eq!(to, expected_to, "encountered invalid TransferSingle.to");
            assert_eq!(
                id, expected_token_id,
                "encountered invalid TransferSingle.id"
            );
            assert_eq!(
                value, expected_value,
                "encountered invalid TransferSingle.value"
            );
        } else {
            panic!("encountered unexpected event kind: expected a TransferSingle event")
        }
    }

    fn assert_transfer_batch_event(
        event: ink_env::test::EmittedEvent,
        expected_operator: AccountId,
        expected_from: AccountId,
        expected_to: AccountId,
        expected_token_ids: &[Id],
        expected_values: &[Balance],
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::TransferBatch(TransferBatch {
            operator,
            from,
            to,
            ids,
            values,
        }) = decoded_event
        {
            assert_eq!(
                operator, expected_operator,
                "encountered invalid TransferBatch.operator"
            );
            assert_eq!(
                from, expected_from,
                "encountered invalid TransferBatch.from"
            );
            assert_eq!(to, expected_to, "encountered invalid TransferBatch.to");
            assert_eq!(
                ids, expected_token_ids,
                "encountered invalid TransferBatch.ids"
            );
            assert_eq!(
                values, expected_values,
                "encountered invalid TransferBatch.values"
            );
        } else {
            panic!("encountered unexpected event kind: expected a TransferBatch event")
        }
    }

    fn assert_approve_event(
        event: ink_env::test::EmittedEvent,
        expected_owner: AccountId,
        expected_operator: AccountId,
        expected_approved: bool,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::ApprovalForAll(ApprovalForAll {
            owner,
            operator,
            approved,
        }) = decoded_event
        {
            assert_eq!(
                owner, expected_owner,
                "encountered invalid ApprovalForAll.owner"
            );
            assert_eq!(
                operator, expected_operator,
                "encountered invalid ApprovalForAll.to"
            );
            assert_eq!(
                approved, expected_approved,
                "encountered invalid ApprovalForAll.approved"
            );
        } else {
            panic!("encountered unexpected event kind: expected a ApprovalForAll event")
        }
    }
}