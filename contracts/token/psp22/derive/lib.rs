#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive_macro::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_psp22_storage, PSP22Storage, PSP22StorageField);
declare_derive_storage_trait!(
    derive_psp22metadata_storage,
    PSP22MetadataStorage,
    PSP22MetadataStorageField
);
declare_derive_storage_trait!(
    derive_psp22_wrapped_storage,
    PSP22WrappedStorage,
    PSP22WrappedStorageField
);
