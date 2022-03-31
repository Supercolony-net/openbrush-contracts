#![cfg_attr(not(feature = "std"), no_std)]
extern crate proc_macro;

#[allow(unused_imports)]
use brush_derive::declare_derive_storage_trait;

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

// PSP34
#[cfg(feature = "psp34")]
declare_derive_storage_trait!(derive_psp34_storage, PSP34Storage, PSP34StorageField);
#[cfg(feature = "psp34")]
declare_derive_storage_trait!(
    derive_psp34_metadata_storage,
    PSP34MetadataStorage,
    PSP34MetadataStorageField
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

// AccessControl
#[cfg(feature = "access_control")]
declare_derive_storage_trait!(
    derive_access_control_storage,
    AccessControlStorage,
    AccessControlStorageField
);

// Ownable
#[cfg(feature = "ownable")]
declare_derive_storage_trait!(derive_ownable_storage, OwnableStorage, OwnableStorageField);

// PaymentSplitter
#[cfg(feature = "payment_splitter")]
declare_derive_storage_trait!(
    derive_payment_storage,
    PaymentSplitterStorage,
    PaymentSplitterStorageField
);

// ReentrancyGuard
#[cfg(feature = "reentrancy_guard")]
declare_derive_storage_trait!(
    derive_reentrancy_storage,
    ReentrancyGuardStorage,
    ReentrancyGuardStorageField
);

// Pausable
#[cfg(feature = "pausable")]
declare_derive_storage_trait!(pausable_storage, PausableStorage, PausableStorageField);

// TimelockController
#[cfg(feature = "timelock_controller")]
declare_derive_storage_trait!(
    derive_timelock_controller_storage,
    TimelockControllerStorage,
    TimelockControllerStorageField
);

// Proxy
#[cfg(feature = "proxy")]
declare_derive_storage_trait!(derive_proxy_storage, ProxyStorage, ProxyStorageField);
// Diamond
#[cfg(feature = "diamond")]
declare_derive_storage_trait!(derive_diamond_storage, DiamondStorage, DiamondStorageField);
#[cfg(feature = "diamond")]
declare_derive_storage_trait!(
    derive_diamond_loupe_storage,
    DiamondLoupeStorage,
    DiamondLoupeStorageField
);
