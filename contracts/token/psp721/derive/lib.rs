#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive_macro::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_psp721_storage, PSP721Storage, PSP721StorageField);
declare_derive_storage_trait!(
    derive_psp721_metadata_storage,
    PSP721MetadataStorage,
    PSP721MetadataStorageField
);
