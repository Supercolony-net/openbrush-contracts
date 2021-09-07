#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive_macro::declare_derive_storage_trait;

declare_derive_storage_trait!(
    derive_timelock_controller_storage,
    TimelockControllerStorage,
    TimelockControllerStorageField
);
