pub mod burnable;
pub mod flashmint;
pub mod metadata;
pub mod mintable;
pub mod stub;
pub mod wrapper;

pub use self::stub::{
    burnable::PSP22Burnable as PSP22BurnableStub,
    metadata::PSP22Metadata as PSP22MetadataStub,
    mintable::PSP22Mintable as PSP22MintableStub,
    psp22flashmint::PSP22FlashMint as PSP22FlashMintStub,
    psp3156_flash_borrower::PSP3156FlashBorrower as PSP3156FlashBorrowerStub,
    wrapper::PSP22Wrapper as PSP22WrapperStub,
};
