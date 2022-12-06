use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Storage error")]
    StorageErr {},

    #[error("User Already Registered")]
    UserAlreadyRegistered { name: String },

    #[error("Bech32 decoding failed for addr: {addr:?}")]
    Bech32DecodingErr { addr: String },

    #[error("Address hash method not supported")]
    HashMethodNotSupported {},

    #[error("Signature mismatch")]
    SignatureMisMatch {},

    #[error("signature already exists")]
    SigntaureAlreadyExists {},

    #[error("Bech32 prefix mismatch between prefix: {prefix:?} and addr: {addr:?}")]
    Bech32PrefixMismatch { prefix: String, addr: String },

    #[error("Bech32 Address not set for name: {name:?}, address: {address:?}")]
    Bech32AddressNotSet { name: String, address: String },

    #[error("Replace address is not set for name: {name:?}, address: {address:?}")]
    ReplacePrimaryAddressNotSet { name: String, address: String },
}
