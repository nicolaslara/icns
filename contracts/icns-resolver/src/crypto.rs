use crate::state::SIGNATURE;
use base64::encode as base64_encode;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use sha3::Keccak256;
use std::ops::Deref;
use subtle_encoding::bech32::encode as bech32_encode;

use cosmwasm_std::{Binary, Deps, Response};

use crate::{msg::Adr36Info, ContractError};

pub fn adr36_verification(
    deps: Deps,
    name: String,
    sender: String,
    bech32_prefix: String,
    adr36_info: Adr36Info,
    chain_id: String,
    contract_address: String,
) -> Result<Response, ContractError> {
    // check if signature already exists
    let signtaure = SIGNATURE.may_load(deps.storage, adr36_info.signature.as_slice())?;
    if signtaure.is_some() {
        return Err(ContractError::SigntaureAlreadyExists {});
    }

    let message = create_adr36_message(
        name,
        bech32_prefix,
        sender,
        adr36_info.signer_bech32_address,
        chain_id,
        contract_address,
        adr36_info.signature_salt.u128(),
    );

    let message_bytes = message.as_bytes();
    let message_hash = Sha256::digest(message_bytes);

    // verify signature using secp256k1
    let verified_result = deps
        .api
        .secp256k1_verify(&message_hash, &adr36_info.signature, &adr36_info.pub_key)
        .map_err(|_| ContractError::SignatureMisMatch {})?;
    if !verified_result {
        return Err(ContractError::SignatureMisMatch {});
    }

    Ok(Response::default())
}

pub fn cosmos_pubkey_to_bech32_address(pub_key: Binary, bech32_prefix: String) -> String {
    let decoded_pub_key = pub_key.as_slice();
    let sha256 = Sha256::digest(decoded_pub_key);
    let result = Ripemd160::digest(sha256);

    bech32_encode(&bech32_prefix, result.deref())
}

pub fn eth_pubkey_to_bech32_address(pub_key: Binary, bech32_prefix: String) -> String {
    // remove first byte(prefix) from public key
    let xy = &pub_key.as_slice()[1..];

    let hashed = Keccak256::digest(xy).as_slice()[12..].to_vec();

    bech32_encode(&bech32_prefix, hashed)
}

pub fn create_adr36_message(
    name: String,
    bech32_prefix: String,
    sender: String,
    signer_bech32_address: String,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> String {
    let message_prefix = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"";
    let data = create_adr36_data(
        name,
        bech32_prefix,
        sender,
        chain_id,
        contract_address,
        signature_salt,
    );
    let signer_prefix = "\",\"signer\":\"";
    let message_suffix = "\"}}],\"sequence\":\"0\"}";
    let message = format!(
        "{}{}{}{}{}",
        message_prefix, data, signer_prefix, signer_bech32_address, message_suffix
    );

    message
}

pub fn create_adr36_data(
    name: String,
    bech32_prefix: String,
    sender: String,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> String {
    let icns = name + "." + &bech32_prefix;
    let address = sender;
    let salt = signature_salt.to_string();

    let data_string = format!(
        "The following is the information for ICNS registration for {}.

Chain id: {}
Contract Address: {}
Owner: {}
Salt: {}",
        icns, chain_id, contract_address, address, salt
    );

    base64_encode(data_string)
}
