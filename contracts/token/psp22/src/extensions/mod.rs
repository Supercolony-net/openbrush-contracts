pub mod burnable;
pub mod metadata;
pub mod mintable;
pub mod stub;

pub use self::stub::{
    burnable::PSP22Burnable,
    metadata::PSP22Metadata,
    mintable::PSP22Mintable,
};
