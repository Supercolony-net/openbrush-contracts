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
#[cfg(feature = "psp35")]
#[openbrush::contract]
mod psp35_metadata {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;
    use openbrush_contracts::psp35::extensions::metadata::*;

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    #[derive(Default, SpreadAllocate, PSP35Storage, PSP35MetadataStorage)]
    #[ink(storage)]
    pub struct PSP35Struct {
        #[PSP35StorageField]
        psp35: PSP35Data,
        #[PSP35MetadataStorageField]
        metadata: PSP35MetadataData,
    }

    impl PSP35 for PSP35Struct {}

    impl PSP35Metadata for PSP35Struct {}

    impl PSP35MetadataInternal for PSP35Struct {
        fn _emit_attribute_set_event(&self, _id: &Id, _key: &Vec<u8>, _data: &Vec<u8>) {
            self.env().emit_event(AttributeSet {
                id: _id.clone(),
                key: _key.clone(),
                data: _data.clone(),
            });
        }
    }

    impl PSP35Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), PSP35Error> {
            self._set_attribute(&id, &key, &data)
        }
    }

    #[ink::test]
    fn metadata_works() {
        let mut nft = PSP35Struct::new();

        assert!(nft.set_attribute(Id::U128(1), vec![0u8, 0u8], vec![1u8, 0u8]).is_ok());

        assert_eq!(nft.get_attribute(Id::U128(1), vec![0u8, 0u8]), Some(vec![1u8, 0u8]));
    }
}
