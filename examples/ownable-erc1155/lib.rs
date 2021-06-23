#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod ownable_erc1155 {
    use erc1155::{
        traits::{IErc1155, Id, IErc1155MetadataURI, IErc1155Receiver, Erc1155ReceiverError},
        impls::{Erc1155Storage, Erc1155, Erc1155MetadataURI}
    };
    use ownable::{
        traits::{IOwnable, OwnableError},
        impls::{OwnableStorage, Ownable}
    };
    use brush::{
        traits::{InkStorage},
        iml_getters,
    };
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
    };
    use ink_lang::{Env, EmitEvent};
    use ink_prelude::{vec::Vec};

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

    const ZERO_ADDRESS: [u8; 32] = [0; 32];

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc1155Struct {
        // Fields of Erc1155Storage
        balances: StorageHashMap<(Id, AccountId), Balance>,
        operator_approval: StorageHashMap<(AccountId, AccountId), bool>,

        // Fields of OwnableStorage
        owner: AccountId,
    }

    impl OwnableStorage for Erc1155Struct {
        iml_getters!(owner, _owner, _owner_mut, AccountId);
    }

    impl InkStorage for Erc1155Struct {}
    impl Erc1155Storage for Erc1155Struct {
        iml_getters!(balances, _balances, _balances_mut, StorageHashMap<(Id, AccountId), Balance>);
        iml_getters!(operator_approval, _operator_approval, _operator_approval_mut, StorageHashMap<(AccountId, AccountId), bool>);
    }
    impl Erc1155 for Erc1155Struct {
        fn emit_transfer_single_event(&self,
                                      _operator: AccountId, _from: AccountId, _to: AccountId, _id: Id, _amount: Balance) {
            self.env().emit_event(TransferSingle {
                operator: _operator,
                from: _from,
                to: _to,
                id: _id,
                value: _amount,
            });
        }

        fn emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner: _owner,
                operator: _operator,
                approved: _approved,
            });
        }

        fn emit_transfer_batch_event(&self,
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

    impl Erc1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::_empty()
        }

        #[ink(message)]
        pub fn temp(&self) {}
    }

    impl Ownable for Erc1155Struct {}

    impl_trait!(Erc1155Struct, IErc1155MetadataURI(Erc1155), IErc1155(Erc1155), IOwnable(Ownable));
}