#![feature(min_specialization)]
#[cfg(feature = "psp22")]
#[brush::contract]
mod psp22_metadata {
    /// Imports all the definitions from the outer scope so we can use them here.
    use contracts::psp22::extensions::metadata::*;
    use ink_lang as ink;

    /// A simple PSP-20 contract.
    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22MetadataStorage)]
    pub struct PSP22Struct {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22 for PSP22Struct {}

    impl PSP22Metadata for PSP22Struct {}

    impl PSP22Struct {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimal;
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let token = PSP22Struct::new(Some(String::from("TOKEN")), Some(String::from("TKN")), 18);

        assert_eq!(token.token_name(), Some(String::from("TOKEN")));
        assert_eq!(token.token_symbol(), Some(String::from("TKN")));
        assert_eq!(token.token_decimals(), 18);
    }
}
