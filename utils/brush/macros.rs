#[macro_export]
macro_rules! declare_storage_trait {
    ($trait_name:ident,$data_ty:ty) => {
        pub trait $trait_name: InkStorage + Sized {
            fn get(&self) -> &$data_ty;
            fn get_mut(&mut self) -> &mut $data_ty;
        }
    };
}

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
