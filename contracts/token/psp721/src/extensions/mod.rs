pub mod burnable;
pub mod metadata;
pub mod mintable;
pub mod stub;

pub use self::stub::{
    burnable::PSP721Burnable,
    metadata::PSP721Metadata,
    mintable::PSP721Mintable,
};
