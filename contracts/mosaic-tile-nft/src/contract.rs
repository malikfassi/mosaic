use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, QueryMsg, TileInfoResponse, TileAtPositionResponse,
    TileColorHistoryResponse, EnableUpdatableResponse,
};
use crate::state::{
    Color, ColorHistory, Position, TileMetadata,
    FROZEN_TOKEN_METADATA, ENABLE_UPDATABLE, TILE_METADATA, POSITION_TO_TOKEN, MAX_POSITION,
};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Empty, Addr, Event,
};
use cw2::set_contract_version;
use sg721_base::msg::InstantiateMsg;
use sg721_base::state::TokenInfo;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:mosaic-tile-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Initialize base sg721 contract
    let res = sg721_base::contract::instantiate(deps.branch(), env, info, msg)?;

    // Initialize our custom state
    FROZEN_TOKEN_METADATA.save(deps.storage, &false)?;
    ENABLE_UPDATABLE.save(deps.storage, &true)?; // Enable updates by default for tiles

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Tile-specific messages
        ExecuteMsg::MintTile {
            token_id,
            owner,
            position,
            color,
        } => execute_mint_tile(deps, env, info, token_id, owner, position, color),
        ExecuteMsg::UpdateTileColor {
            token_id,
            color,
        } => execute_update_color(deps, env, info, token_id, color),
        ExecuteMsg::FreezeTokenMetadata {} => execute_freeze_metadata(deps, info),
        ExecuteMsg::EnableUpdatable {} => execute_enable_updatable(deps, info),
        
        // Forward other messages to base contract
        _ => Ok(sg721_base::contract::execute(deps, env, info, msg.into())?),
    }
}

pub fn execute_mint_tile(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    position: Position,
    color: Color,
) -> Result<Response, ContractError> {
    // Cache minter to avoid multiple reads
    let minter = sg721_base::state::MINTER.load(deps.storage)?;
    if info.sender != minter {
        return Err(ContractError::Unauthorized {});
    }

    // Validate position
    if position.x > MAX_POSITION || position.y > MAX_POSITION {
        return Err(ContractError::InvalidPosition {});
    }

    // Check if position is already taken - use indexed query
    if TILE_METADATA.idx.position.has(deps.storage, (position.x, position.y)) {
        return Err(ContractError::PositionTaken {
            x: position.x,
            y: position.y,
        });
    }

    // Create initial metadata
    let metadata = TileMetadata {
        position: position.clone(),
        current_color: color.clone(),
    };

    // Save tile metadata using indexed storage
    TILE_METADATA.save(deps.storage, &token_id, &metadata)?;

    // Mint the NFT using base contract functionality
    let mint_msg = sg721_base::ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner,
        token_uri: None,
        extension: Empty {},
    };
    let res = sg721_base::contract::execute(deps, env.clone(), info.clone(), mint_msg)?;

    // Create color update event
    let color_event = Event::new("tile_color_update")
        .add_attribute("token_id", token_id.clone())
        .add_attribute("color_r", color.r.to_string())
        .add_attribute("color_g", color.g.to_string())
        .add_attribute("color_b", color.b.to_string())
        .add_attribute("updater", info.sender.to_string())
        .add_attribute("timestamp", env.block.time.seconds().to_string());

    Ok(Response::new()
        .add_submessages(res.messages)
        .add_attributes(res.attributes)
        .add_event(color_event)
        .add_attribute("action", "mint_tile")
        .add_attribute("token_id", token_id)
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string()))
}

pub fn execute_update_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    color: Color,
) -> Result<Response, ContractError> {
    // Batch load state flags to minimize reads
    let (frozen, updatable) = (
        FROZEN_TOKEN_METADATA.load(deps.storage)?,
        ENABLE_UPDATABLE.load(deps.storage)?,
    );

    // Check state in memory
    if frozen {
        return Err(ContractError::TokenMetadataFrozen {});
    }
    if !updatable {
        return Err(ContractError::NotEnableUpdatable {});
    }

    // Load token and check ownership/approval
    let token = sg721_base::state::TokenInfo::<Empty>::load(deps.storage, &token_id)?;
    if !token.owner.as_ref().eq(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    // Update tile metadata
    TILE_METADATA.update(deps.storage, &token_id, |metadata| -> StdResult<_> {
        let mut metadata = metadata.ok_or_else(|| StdError::not_found("TileMetadata"))?;
        metadata.current_color = color.clone();
        Ok(metadata)
    })?;

    // Create color update event
    let color_event = Event::new("tile_color_update")
        .add_attribute("token_id", token_id.clone())
        .add_attribute("color_r", color.r.to_string())
        .add_attribute("color_g", color.g.to_string())
        .add_attribute("color_b", color.b.to_string())
        .add_attribute("updater", info.sender.to_string())
        .add_attribute("timestamp", env.block.time.seconds().to_string());

    Ok(Response::new()
        .add_event(color_event)
        .add_attribute("action", "update_color")
        .add_attribute("token_id", token_id))
}

pub fn execute_freeze_metadata(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Only minter can freeze metadata
    let minter = sg721_base::state::MINTER.load(deps.storage)?;
    if info.sender != minter {
        return Err(ContractError::Unauthorized {});
    }

    FROZEN_TOKEN_METADATA.save(deps.storage, &true)?;

    Ok(Response::new().add_attribute("action", "freeze_metadata"))
}

pub fn execute_enable_updatable(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Only minter can enable updates
    let minter = sg721_base::state::MINTER.load(deps.storage)?;
    if info.sender != minter {
        return Err(ContractError::Unauthorized {});
    }

    if ENABLE_UPDATABLE.load(deps.storage)? {
        return Err(ContractError::AlreadyEnableUpdatable {});
    }

    ENABLE_UPDATABLE.save(deps.storage, &true)?;

    Ok(Response::new().add_attribute("action", "enable_updatable"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TileInfo { token_id } => to_binary(&query_tile_info(deps, token_id)?),
        QueryMsg::TileAtPosition { position } => to_binary(&query_tile_at_position(deps, position)?),
        QueryMsg::EnableUpdatable {} => to_binary(&query_enable_updatable(deps)?),
        QueryMsg::FreezeTokenMetadata {} => to_binary(&FROZEN_TOKEN_METADATA.load(deps.storage)?),
        _ => sg721_base::contract::query(deps, env, msg.into()),
    }
}

fn query_tile_info(deps: Deps, token_id: String) -> StdResult<TileInfoResponse> {
    let token = sg721_base::state::TokenInfo::<Empty>::load(deps.storage, &token_id)?;
    let metadata = TILE_METADATA.load(deps.storage, &token_id)?;

    Ok(TileInfoResponse {
        token_id,
        owner: token.owner.to_string(),
        metadata,
    })
}

fn query_tile_at_position(deps: Deps, position: Position) -> StdResult<TileAtPositionResponse> {
    // Use indexed query for efficient position lookup
    let tokens: Vec<String> = TILE_METADATA
        .idx
        .position
        .prefix((position.x, position.y))
        .keys(deps.storage, None, None, Order::Ascending)
        .take(1)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(TileAtPositionResponse {
        token_id: tokens.first().cloned(),
    })
}

fn query_enable_updatable(deps: Deps) -> StdResult<EnableUpdatableResponse> {
    let enabled = ENABLE_UPDATABLE.load(deps.storage)?;
    Ok(EnableUpdatableResponse { enabled })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{
        from_json, to_json_binary, ContractInfoResponse, ContractResult, Empty, OwnedDeps, Querier,
        QuerierResult, QueryRequest, SystemError, SystemResult, WasmQuery,
    };
    use cw721::Cw721Query;
    use sg721::{CollectionInfo, InstantiateMsg};
    use std::marker::PhantomData;

    const CREATOR: &str = "creator";
    const MINTER: &str = "minter";
    const HACKER: &str = "hacker";

    pub fn mock_deps() -> OwnedDeps<MockStorage, MockApi, CustomMockQuerier, Empty> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: CustomMockQuerier::new(MockQuerier::new(&[])),
            custom_query_type: PhantomData,
        }
    }

    pub struct CustomMockQuerier {
        base: MockQuerier,
    }

    impl Querier for CustomMockQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
            let request: QueryRequest<Empty> = match from_json(bin_request) {
                Ok(v) => v,
                Err(e) => {
                    return SystemResult::Err(SystemError::InvalidRequest {
                        error: format!("Parsing query request: {e}"),
                        request: bin_request.into(),
                    })
                }
            };

            self.handle_query(&request)
        }
    }

    impl CustomMockQuerier {
        pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
            match &request {
                QueryRequest::Wasm(WasmQuery::ContractInfo { contract_addr: _ }) => {
                    let mut response = ContractInfoResponse::default();
                    response.code_id = 1;
                    response.creator = CREATOR.to_string();
                    SystemResult::Ok(ContractResult::Ok(to_json_binary(&response).unwrap()))
                }
                _ => self.base.handle_query(request),
            }
        }

        pub fn new(base: MockQuerier<Empty>) -> Self {
            CustomMockQuerier { base }
        }
    }

    fn setup_contract() -> (OwnedDeps<MockStorage, MockApi, CustomMockQuerier, Empty>, Env) {
        let mut deps = mock_deps();
        let env = mock_env();

        // Instantiate contract
        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "MosaicTiles".to_string(),
            symbol: "TILE".to_string(),
            minter: MINTER.to_string(),
            collection_info: CollectionInfo {
                creator: CREATOR.to_string(),
                description: "Mosaic Tile NFTs".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
        };
        instantiate(deps.as_mut(), env.clone(), info, init_msg).unwrap();

        (deps, env)
    }

    #[test]
    fn proper_initialization() {
        let (deps, _) = setup_contract();
        
        // Check that metadata is not frozen initially
        let frozen = FROZEN_TOKEN_METADATA.load(deps.as_ref().storage).unwrap();
        assert!(!frozen);

        // Check that updates are enabled initially
        let updatable = ENABLE_UPDATABLE.load(deps.as_ref().storage).unwrap();
        assert!(updatable);
    }

    #[test]
    fn mint_tile() {
        let (mut deps, env) = setup_contract();
        let token_id = "tile1";
        let owner = "owner";
        
        // Mint a tile
        let mint_msg = ExecuteMsg::MintTile {
            token_id: token_id.to_string(),
            owner: owner.to_string(),
            position: Position { x: 1, y: 1 },
            color: Color { r: 255, g: 0, b: 0 },
        };

        // Only minter can mint
        let unauthorized_info = mock_info(HACKER, &[]);
        let err = execute(
            deps.as_mut(),
            env.clone(),
            unauthorized_info,
            mint_msg.clone(),
        )
        .unwrap_err();
        assert_eq!(err.to_string(), ContractError::Unauthorized {}.to_string());

        // Successful mint
        let minter_info = mock_info(MINTER, &[]);
        let res = execute(deps.as_mut(), env.clone(), minter_info, mint_msg).unwrap();
        assert_eq!(res.attributes.len(), 5); // action, token_id, owner, position_x, position_y

        // Query tile info
        let query_msg = QueryMsg::TileInfo {
            token_id: token_id.to_string(),
        };
        let res: TileInfoResponse = from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.owner, owner);
        assert_eq!(res.metadata.position.x, 1);
        assert_eq!(res.metadata.position.y, 1);
        assert_eq!(res.metadata.current_color.r, 255);

        // Cannot mint at same position
        let mint_msg2 = ExecuteMsg::MintTile {
            token_id: "tile2".to_string(),
            owner: owner.to_string(),
            position: Position { x: 1, y: 1 },
            color: Color { r: 0, g: 255, b: 0 },
        };
        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(MINTER, &[]),
            mint_msg2,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::PositionTaken { x: 1, y: 1 }));
    }

    #[test]
    fn update_tile_color() {
        let (mut deps, env) = setup_contract();
        let token_id = "tile1";
        let owner = "owner";
        
        // First mint a tile
        let mint_msg = ExecuteMsg::MintTile {
            token_id: token_id.to_string(),
            owner: owner.to_string(),
            position: Position { x: 1, y: 1 },
            color: Color { r: 255, g: 0, b: 0 },
        };
        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(MINTER, &[]),
            mint_msg,
        )
        .unwrap();

        // Update color
        let update_msg = ExecuteMsg::UpdateTileColor {
            token_id: token_id.to_string(),
            color: Color { r: 0, g: 255, b: 0 },
        };

        // Only owner can update
        let unauthorized_info = mock_info(HACKER, &[]);
        let err = execute(
            deps.as_mut(),
            env.clone(),
            unauthorized_info,
            update_msg.clone(),
        )
        .unwrap_err();
        assert_eq!(err.to_string(), ContractError::Unauthorized {}.to_string());

        // Successful update
        let owner_info = mock_info(owner, &[]);
        let res = execute(deps.as_mut(), env.clone(), owner_info, update_msg).unwrap();
        assert_eq!(res.attributes.len(), 2); // action, token_id

        // Query updated tile info
        let query_msg = QueryMsg::TileInfo {
            token_id: token_id.to_string(),
        };
        let res: TileInfoResponse = from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.metadata.current_color.g, 255);

        // Cannot update if frozen
        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(MINTER, &[]),
            ExecuteMsg::FreezeTokenMetadata {},
        )
        .unwrap();

        let update_msg2 = ExecuteMsg::UpdateTileColor {
            token_id: token_id.to_string(),
            color: Color { r: 0, g: 0, b: 255 },
        };
        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(owner, &[]),
            update_msg2,
        )
        .unwrap_err();
        assert_eq!(err.to_string(), ContractError::TokenMetadataFrozen {}.to_string());
    }

    #[test]
    fn query_by_position() {
        let (mut deps, env) = setup_contract();
        let token_id = "tile1";
        
        // Mint a tile
        let mint_msg = ExecuteMsg::MintTile {
            token_id: token_id.to_string(),
            owner: "owner".to_string(),
            position: Position { x: 2, y: 3 },
            color: Color { r: 255, g: 0, b: 0 },
        };
        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(MINTER, &[]),
            mint_msg,
        )
        .unwrap();

        // Query existing position
        let query_msg = QueryMsg::TileAtPosition {
            position: Position { x: 2, y: 3 },
        };
        let res: TileAtPositionResponse = from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.token_id, Some(token_id.to_string()));

        // Query empty position
        let query_msg = QueryMsg::TileAtPosition {
            position: Position { x: 5, y: 5 },
        };
        let res: TileAtPositionResponse = from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.token_id, None);
    }
}
