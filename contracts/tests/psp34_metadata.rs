#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34_metadata {
    use contracts::psp34::extensions::metadata::*;
    use ink_lang as ink;
    use ink_prelude::string::String;

    #[derive(Default, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
    }

    impl PSP34Metadata for PSP34Struct {}

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            instance._init_with_metadata(name, symbol);
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let nft = PSP34Struct::new(Some(String::from("TOKEN")), Some(String::from("TKN")));

        assert_eq!(nft.name(), Some(String::from("TOKEN")));
        assert_eq!(nft.symbol(), Some(String::from("TKN")));
        assert_eq!(nft.uri([1; 32]), None);
    }
}
