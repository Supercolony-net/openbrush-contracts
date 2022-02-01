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
        pub fn new(id: Id, key: String, val: String) -> Self {
            let mut instance = Self::default();
            instance._set_attribute(id, key.into_bytes(), val.into_bytes());
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let id = Id::U8(1u8);
        let nft = PSP34Struct::new(id.clone(), String::from("KEY"), String::from("VAL"));

        assert_eq!(
            nft.get_attribute(id.clone(), String::from("KEY").into_bytes()),
            Some(String::from("VAL").into_bytes())
        );
    }
}
