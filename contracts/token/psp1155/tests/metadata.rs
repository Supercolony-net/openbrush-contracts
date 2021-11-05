#[cfg(test)]
#[brush::contract]
mod metadata {
    use ink_lang as ink;
    use psp1155::{
        extensions::metadata::*,
        traits::*,
    };

    #[derive(Default, PSP1155Storage, PSP1155MetadataStorage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
    }

    impl PSP1155 for PSP1155Struct {
        // Don't do cross call in test
        fn _do_safe_transfer_acceptance_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _amount: Balance,
            _data: Vec<u8>,
        ) -> Result<(), psp1155::traits::PSP1155Error> {
            Ok(())
        }

        // Don't do cross call in test
        fn _do_batch_safe_transfer_acceptance_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _ids_to_amounts: Vec<(Id, Balance)>,
            _data: Vec<u8>,
        ) -> Result<(), psp1155::traits::PSP1155Error> {
            Ok(())
        }
    }

    impl PSP1155Metadata for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new(uri: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.uri = uri;
            instance
        }
    }

    #[ink::test]
    fn metadata_works() {
        let nft = PSP1155Struct::new(Some(String::from("https://www.supercolony.net/")));

        assert_eq!(nft.uri([0; 32]), Some(String::from("https://www.supercolony.net/")));
    }
}
