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

/// This `macro_rule` defines the storage trait. It accepts only one argument -
/// the name of the storage trait.
///
/// An example of the usage of this macro can be found in any contract implemented by this library.
/// For example [OwnableStorage](ownable::OwnableStorage).
#[macro_export]
macro_rules! declare_storage_trait {
    ($trait_name:ident) => {
        pub trait $trait_name: ::openbrush::traits::InkStorage {
            type Data;
            fn get(&self) -> &Self::Data;
            fn get_mut(&mut self) -> &mut Self::Data;
        }
    };
}

/// This `macro_rule` manually generates implementation for storage trait.
/// You can use this macro to generate the implementation for the storage trait
/// if you don't want to use derive macro, or if it is not created.
///
/// The first argument is the name of the storage trait.
/// The second argument is the name of the struct, for which you are creating the impl section.
/// The third argument is the name of the field, which will be returned by the trait's implementation.
/// The fourth argument is the type of storage data, which will be returned.
#[macro_export]
macro_rules! impl_storage_trait {
    ($trait_name:ident,$struct_name:ident,$field:ident,$data_ty:ty) => {
        impl $trait_name for $struct_name {
            type Data = $data_ty;
            fn get(&self) -> &Self::Data {
                &self.$field
            }

            fn get_mut(&mut self) -> &mut Self::Data {
                &mut self.$field
            }
        }
    };
}
