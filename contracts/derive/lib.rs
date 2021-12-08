#![cfg_attr(not(feature = "std"), no_std)]
extern crate proc_macro;

#[allow(unused_imports)]
use brush_derive_macro::declare_derive_storage_trait;

// PSP22
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(derive_psp22_storage, PSP22Storage, PSP22StorageField);
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(
    derive_psp22metadata_storage,
    PSP22MetadataStorage,
    PSP22MetadataStorageField
);
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(
    derive_psp22wrapper_storage,
    PSP22WrapperStorage,
    PSP22WrapperStorageField
);
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(
    derive_psp22token_timelock_storage,
    PSP22TokenTimelockStorage,
    PSP22TokenTimelockStorageField
);

// PSP721
#[cfg(feature = "psp721")]
declare_derive_storage_trait!(derive_psp721_storage, PSP721Storage, PSP721StorageField);
#[cfg(feature = "psp721")]
declare_derive_storage_trait!(
    derive_psp721_metadata_storage,
    PSP721MetadataStorage,
    PSP721MetadataStorageField
);

// PSP1155
#[cfg(feature = "psp1155")]
declare_derive_storage_trait!(derive_psp1155_storage, PSP1155Storage, PSP1155StorageField);
#[cfg(feature = "psp1155")]
declare_derive_storage_trait!(
    derive_psp1155_metadata_storage,
    PSP1155MetadataStorage,
    PSP1155MetadataStorageField
);
