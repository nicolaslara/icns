use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::{
    checks::check_admin,
    error::ContractError,
    state::{Config, CONFIG},
    ICNSNameNFTContract,
};

pub fn add_admin(admin: &str, deps: DepsMut) -> Result<Response, ContractError> {
    // check that admin does not already exist
    let config = CONFIG.load(deps.storage)?;
    for existing_admin in config.admins {
        if existing_admin == deps.api.addr_validate(admin)? {
            return Err(cw721_base::ContractError::Unauthorized {}.into());
        }
    }

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        let mut admins = config.admins;
        admins.push(deps.api.addr_validate(admin)?);
        Ok(Config { admins, ..config })
    })?;
    Ok(Response::new()
        .add_attribute("method", "add_admin")
        .add_attribute("admin", admin))
}

pub fn remove_admin(admin: &str, deps: DepsMut) -> Result<Response, ContractError> {
    let admin_addr = deps.api.addr_validate(admin)?;
    // check that admin exists
    check_admin(deps.as_ref(), &admin_addr)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        let mut admins = config.admins;
        admins.retain(|x| x != &admin_addr);
        Ok(Config { admins, ..config })
    })?;

    Ok(Response::new()
        .add_attribute("method", "remove_admin")
        .add_attribute("admin", admin))
}

pub fn set_transferrable(transferrable: bool, deps: DepsMut) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            transferrable,
            ..config
        })
    })?;
    Ok(Response::new()
        .add_attribute("method", "set_transferrable")
        .add_attribute("transferrable", transferrable.to_string()))
}

pub fn set_minter_address(minter_address: &str, deps: DepsMut) -> Result<Response, ContractError> {
    let name_nft = ICNSNameNFTContract::default();
    let minter = deps.api.addr_validate(minter_address)?;

    name_nft.minter.save(deps.storage, &minter)?;

    Ok(Response::new()
        .add_attribute("method", "set_minter_address")
        .add_attribute("minter_address", minter_address))
}
