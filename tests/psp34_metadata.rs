// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[openbrush::contract]
mod psp34_metadata {
    use ink_lang as ink;
    use ink_prelude::string::String;
    use openbrush::contracts::psp34::extensions::metadata::*;

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
