#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive_macro::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_psp1155_storage, PSP1155Storage, PSP1155StorageField);
declare_derive_storage_trait!(
    derive_psp1155_metadata_storage,
    PSP1155MetadataStorage,
    PSP1155MetadataStorageField
);
