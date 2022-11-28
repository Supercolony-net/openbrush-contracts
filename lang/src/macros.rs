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

/// Return the hash of the format!("{}::{}", ::core::module_path!(), struct_name).
/// It cam be used to generate unique storage key of the struct.
#[macro_export]
macro_rules! storage_unique_key {
    ($struct:ident) => {
        $crate::utils::ConstHasher::hash($crate::utils::const_format::concatcp!(
            ::core::module_path!(),
            "::",
            ::core::stringify!($struct)
        ))
    };
}

#[test]
fn correct_storage_key() {
    use crate::{
        traits::OccupyStorage,
        utils::ConstHasher,
    };

    mod contracts {
        pub mod psp22 {
            use crate::traits::OccupyStorage;
            pub struct Data;

            impl OccupyStorage for Data {
                const KEY: u32 = storage_unique_key!(Data);
            }
        }

        pub mod psp34 {
            use crate::traits::OccupyStorage;
            pub struct Data;

            impl OccupyStorage for Data {
                const KEY: u32 = storage_unique_key!(Data);
            }
        }
    }

    let expected_hash_psp22 = ConstHasher::hash("openbrush_lang::macros::contracts::psp22::Data");
    assert_eq!(expected_hash_psp22, <contracts::psp22::Data as OccupyStorage>::KEY);

    let expected_hash_psp34 = ConstHasher::hash("openbrush_lang::macros::contracts::psp34::Data");
    assert_eq!(expected_hash_psp34, <contracts::psp34::Data as OccupyStorage>::KEY);
}
