use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cw721_base::{
    msg::{ExecuteMsg as Cw721ExecuteMsg, QueryMsg as Cw721QueryMsg},
    state::TokenInfo,
    InstantiateMsg as Cw721InstantiateMsg,
};

use crate::{
    error::ContractError,
    execute::{execute_batch_set_pixels, execute_mint_tile, execute_set_pixel_color},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{
        query_batch_tile_pixels, query_mosaic_state, query_pixel_state, query_pixels_state,
        query_tile_pixels, query_tile_state, query_tiles_state,
    },
    state::{
        Cw721StorageType, TileMetadata, DEVELOPER, DEVELOPER_FEE, MINTER, OWNER_FEE, TOKEN_COUNT,
    },
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Store mosaic-specific configuration first
    MINTER.save(deps.storage, &msg.minter)?;
    DEVELOPER.save(deps.storage, &msg.developer)?;
    DEVELOPER_FEE.save(deps.storage, &msg.developer_fee)?;
    OWNER_FEE.save(deps.storage, &msg.owner_fee)?;
    TOKEN_COUNT.save(deps.storage, &0u64)?;

    // Initialize CW721 contract
    let contract = Cw721StorageType::default();
    let cw721_msg = Cw721InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: msg.minter.clone(),
    };
    contract.instantiate(deps.branch(), env, info.clone(), cw721_msg)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("minter", msg.minter)
        .add_attribute("developer", msg.developer))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Cw721(msg) => {
            let contract = Cw721StorageType::default();
            let res = contract.execute(deps, env, info, *msg)?;
            Ok(Response::<Empty>::new()
                .add_attributes(res.attributes)
                .add_events(res.events))
        }
        ExecuteMsg::MintTile { tile_id, owner } => {
            // Execute mint
            execute_mint_tile(deps.branch(), env, info.clone(), tile_id, owner.clone())
        }
        ExecuteMsg::SetPixelColor { pixel_id, color } => {
            execute_set_pixel_color(deps, env, info, pixel_id, color)
        }
        ExecuteMsg::BatchSetPixels { updates } => {
            execute_batch_set_pixels(deps, env, info, updates)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Cw721(msg) => {
            let contract = Cw721StorageType::default();
            let msg: Cw721QueryMsg<Empty> = match *msg {
                Cw721QueryMsg::OwnerOf {
                    token_id,
                    include_expired,
                } => Cw721QueryMsg::OwnerOf {
                    token_id,
                    include_expired,
                },
                Cw721QueryMsg::Approval {
                    token_id,
                    spender,
                    include_expired,
                } => Cw721QueryMsg::Approval {
                    token_id,
                    spender,
                    include_expired,
                },
                Cw721QueryMsg::Approvals {
                    token_id,
                    include_expired,
                } => Cw721QueryMsg::Approvals {
                    token_id,
                    include_expired,
                },
                Cw721QueryMsg::AllOperators {
                    owner,
                    include_expired,
                    start_after,
                    limit,
                } => Cw721QueryMsg::AllOperators {
                    owner,
                    include_expired,
                    start_after,
                    limit,
                },
                Cw721QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
                Cw721QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
                Cw721QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
                Cw721QueryMsg::AllNftInfo {
                    token_id,
                    include_expired,
                } => Cw721QueryMsg::AllNftInfo {
                    token_id,
                    include_expired,
                },
                Cw721QueryMsg::Tokens {
                    owner,
                    start_after,
                    limit,
                } => Cw721QueryMsg::Tokens {
                    owner,
                    start_after,
                    limit,
                },
                Cw721QueryMsg::AllTokens { start_after, limit } => {
                    Cw721QueryMsg::AllTokens { start_after, limit }
                }
                Cw721QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
                Cw721QueryMsg::Operator {
                    owner,
                    operator,
                    include_expired,
                } => Cw721QueryMsg::Operator {
                    owner,
                    operator,
                    include_expired,
                },
                Cw721QueryMsg::Extension { msg: _ } => Cw721QueryMsg::Extension { msg: Empty {} },
                Cw721QueryMsg::Ownership {} => Cw721QueryMsg::Ownership {},
            };
            contract.query(deps, env, msg)
        }
        QueryMsg::TileState { tile_id } => to_json_binary(&query_tile_state(deps, tile_id)?),
        QueryMsg::TilesState { tile_ids } => to_json_binary(&query_tiles_state(deps, tile_ids)?),
        QueryMsg::PixelState { pixel_id } => to_json_binary(&query_pixel_state(deps, pixel_id)?),
        QueryMsg::PixelsState {
            pixel_ids,
            start_after,
            limit,
        } => to_json_binary(&query_pixels_state(deps, pixel_ids, start_after, limit)?),
        QueryMsg::MosaicState {} => to_json_binary(&query_mosaic_state(deps)?),
        QueryMsg::TilePixels { tile_id } => to_json_binary(&query_tile_pixels(deps, tile_id)?),
        QueryMsg::BatchTilePixels { tile_ids } => {
            to_json_binary(&query_batch_tile_pixels(deps, tile_ids)?)
        }
    }
}
