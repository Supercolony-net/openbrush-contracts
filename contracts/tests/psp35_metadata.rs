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
#[brush::contract]
mod psp35_metadata {
    use contracts::psp35::extensions::metadata::*;
    use ink_lang as ink;

    #[derive(Default, PSP35Storage, PSP35MetadataStorage)]
    #[ink(storage)]
    pub struct PSP35Struct {
        #[PSP35StorageField]
        psp35: PSP35Data,
        #[PSP35MetadataStorageField]
        metadata: PSP35MetadataData,
    }

    impl PSP35 for PSP35Struct {}

    impl PSP35Metadata for PSP35Struct {}

    impl PSP35Struct {
        #[ink(constructor)]
        pub fn new(uri: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.uri = uri;
            instance
        }
    }

    #[ink::test]
    fn metadata_works() {
        let nft = PSP35Struct::new(Some(String::from("https://www.supercolony.net/")));

        assert_eq!(nft.uri([0; 32]), Some(String::from("https://www.supercolony.net/")));
    }
}
