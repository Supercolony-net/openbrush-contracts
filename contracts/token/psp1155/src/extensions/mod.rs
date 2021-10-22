pub mod burnable;
pub mod metadata;
pub mod mintable;
pub mod stub;

pub use self::stub::{
    burnable::PSP1155Burnable,
    metadata::PSP1155Metadata,
    mintable::PSP1155Mintable,
};
