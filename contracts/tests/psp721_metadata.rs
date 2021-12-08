#![feature(min_specialization)]
#[cfg(feature = "psp721")]
#[brush::contract]
mod psp721_metadata {
    use contracts::psp721::extensions::metadata::*;
    use ink_lang as ink;
    use ink_prelude::string::String;

    #[derive(Default, PSP721Storage, PSP721MetadataStorage)]
    #[ink(storage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[PSP721MetadataStorageField]
        metadata: PSP721MetadataData,
    }

    impl PSP721Metadata for PSP721Struct {}

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            instance._init_with_metadata(name, symbol);
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let nft = PSP721Struct::new(Some(String::from("TOKEN")), Some(String::from("TKN")));

        assert_eq!(nft.name(), Some(String::from("TOKEN")));
        assert_eq!(nft.symbol(), Some(String::from("TKN")));
        assert_eq!(nft.uri([1; 32]), None);
    }
}
