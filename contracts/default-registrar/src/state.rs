use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

use cw_storage_plus::Item;
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // name_nft address to send msg to
    pub name_nft: Addr,
    // verifier defines the pub key of the verifier who can call this contract
    pub verifiers: Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
