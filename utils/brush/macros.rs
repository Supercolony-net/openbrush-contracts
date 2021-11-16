/// This `macro_rule` defines the storage trait.
///
/// The first argument is the name of the storage trait.
/// The second argument is the type of storage data, which will be returned by this trait.
///
/// An example of the usage of this macro can be found in any contract implemented by this library.
/// For example [OwnableStorage](ownable::OwnableStorage).
#[macro_export]
macro_rules! declare_storage_trait {
    ($trait_name:ident,$data_ty:ty) => {
        #[::brush::trait_definition]
        pub trait $trait_name: InkStorage {
            fn get(&self) -> &$data_ty;
            fn get_mut(&mut self) -> &mut $data_ty;
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
            fn get(&self) -> &$data_ty {
                &self.$field
            }

            fn get_mut(&mut self) -> &mut $data_ty {
                &mut self.$field
            }
        }
    };
}
