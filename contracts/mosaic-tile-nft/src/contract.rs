use crate::error::ContractError;
use crate::msg::{
    EnableUpdatableResponse, ExecuteMsg, QueryMsg, TileAtPositionResponse, TileInfoResponse,
};
use crate::state::{
    Color, Position, TileMetadata, ENABLE_UPDATABLE, FROZEN_TOKEN_METADATA, MAX_POSITION,
    TILE_METADATA,
};

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Order, Response,
    StdError, StdResult,
};
use cw2;
use cw721_base::{
    entry::{execute as cw721_execute, instantiate as cw721_instantiate, query as cw721_query},
    msg::InstantiateMsg,
    ExecuteMsg as Cw721ExecuteMsg, QueryMsg as Cw721QueryMsg,
};

const CONTRACT_NAME: &str = "crates.io:mosaic-tile-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let res = cw721_instantiate(deps.branch(), env, info, msg)?;
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
        ExecuteMsg::MintTile {
            token_id,
            owner,
            position,
            color,
        } => execute_mint_tile(deps, env, info, token_id, owner, position, color),
        ExecuteMsg::UpdateTileColor { token_id, color } => {
            execute_update_color(deps, env, info, token_id, color)
        }
        ExecuteMsg::FreezeTokenMetadata {} => execute_freeze_metadata(deps, info),
        ExecuteMsg::EnableUpdatable {} => execute_enable_updatable(deps, info),
        _ => {
            let cw721_msg = match msg {
                ExecuteMsg::TransferNft {
                    recipient,
                    token_id,
                } => cw721_base::ExecuteMsg::TransferNft {
                    recipient,
                    token_id,
                },
                ExecuteMsg::SendNft {
                    contract,
                    token_id,
                    msg,
                } => cw721_base::ExecuteMsg::SendNft {
                    contract,
                    token_id,
                    msg,
                },
                ExecuteMsg::Approve {
                    spender,
                    token_id,
                    expires,
                } => cw721_base::ExecuteMsg::Approve {
                    spender,
                    token_id,
                    expires,
                },
                ExecuteMsg::Revoke { spender, token_id } => {
                    cw721_base::ExecuteMsg::Revoke { spender, token_id }
                }
                ExecuteMsg::ApproveAll { operator, expires } => {
                    cw721_base::ExecuteMsg::ApproveAll { operator, expires }
                }
                ExecuteMsg::RevokeAll { operator } => {
                    cw721_base::ExecuteMsg::RevokeAll { operator }
                }
                _ => unreachable!("All other cases must be handled above"),
            };
            Ok(cw721_execute(deps, env, info, cw721_msg)?)
        }
    }
}

pub fn execute_mint_tile(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    token_id: String,
    owner: String,
    position: Position,
    color: Color,
) -> Result<Response, ContractError> {
    // Check if position is within bounds
    if position.x > MAX_POSITION || position.y > MAX_POSITION {
        return Err(ContractError::PositionOutOfBounds {
            x: position.x,
            y: position.y,
        });
    }

    // Check if position is already taken
    if TILE_METADATA
        .idx
        .position
        .prefix((position.x, position.y))
        .range(deps.storage, None, None, Order::Ascending)
        .next()
        .transpose()?
        .is_some()
    {
        return Err(ContractError::PositionTaken {
            x: position.x,
            y: position.y,
        });
    }

    // Create mint message for base contract
    let mint_msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner,
        token_uri: None,
        extension: TileMetadata {
            position,
            current_color: color.clone(),
        },
    };

    // Execute mint on base contract
    let res = cw721_execute(deps, env.clone(), _info.clone(), mint_msg)?;

    // Create color update event
    let color_event = Event::new("tile_color_update")
        .add_attribute("token_id", token_id)
        .add_attribute("color_r", color.r.to_string())
        .add_attribute("color_g", color.g.to_string())
        .add_attribute("color_b", color.b.to_string());

    Ok(Response::new()
        .add_event(color_event)
        .add_submessages(res.messages)
        .add_attributes(res.attributes))
}

pub fn execute_update_color(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
    color: Color,
) -> Result<Response, ContractError> {
    // Check if metadata is frozen
    if FROZEN_TOKEN_METADATA.load(deps.storage)? {
        return Err(ContractError::TokenMetadataFrozen {});
    }

    // Check if updates are enabled
    if !ENABLE_UPDATABLE.load(deps.storage)? {
        return Err(ContractError::TokenMetadataFrozen {});
    }

    // Load token metadata
    TILE_METADATA.update(deps.storage, &token_id, |metadata| -> StdResult<_> {
        let mut metadata = metadata.ok_or_else(|| StdError::not_found("TileMetadata"))?;
        metadata.current_color = color.clone();
        Ok(metadata)
    })?;

    // Create color update event
    let color_event = Event::new("tile_color_update")
        .add_attribute("token_id", token_id)
        .add_attribute("color_r", color.r.to_string())
        .add_attribute("color_g", color.g.to_string())
        .add_attribute("color_b", color.b.to_string());

    Ok(Response::new().add_event(color_event))
}

pub fn execute_freeze_metadata(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Only contract admin can freeze metadata
    if info.sender != deps.api.addr_validate("admin")? {
        return Err(ContractError::Unauthorized {});
    }

    // Check if already frozen
    if FROZEN_TOKEN_METADATA.load(deps.storage)? {
        return Err(ContractError::TokenMetadataAlreadyFrozen {});
    }

    // Freeze metadata
    FROZEN_TOKEN_METADATA.save(deps.storage, &true)?;

    Ok(Response::new().add_attribute("action", "freeze_metadata"))
}

pub fn execute_enable_updatable(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Only contract admin can enable updates
    if info.sender != deps.api.addr_validate("admin")? {
        return Err(ContractError::Unauthorized {});
    }

    // Check if already enabled
    if ENABLE_UPDATABLE.load(deps.storage)? {
        return Err(ContractError::AlreadyEnableUpdatable {});
    }

    // Enable updates
    ENABLE_UPDATABLE.save(deps.storage, &true)?;

    Ok(Response::new().add_attribute("action", "enable_updatable"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TileInfo { token_id } => to_json_binary(&query_tile_info(deps, token_id)?),
        QueryMsg::TileAtPosition { position } => {
            to_json_binary(&query_tile_at_position(deps, position)?)
        }
        QueryMsg::EnableUpdatable {} => to_json_binary(&query_enable_updatable(deps)?),
        QueryMsg::FreezeTokenMetadata {} => {
            to_json_binary(&FROZEN_TOKEN_METADATA.load(deps.storage)?)
        }
        _ => {
            let cw721_msg = match msg {
                QueryMsg::OwnerOf {
                    token_id,
                    include_expired,
                } => cw721_base::QueryMsg::<Empty>::OwnerOf {
                    token_id,
                    include_expired,
                },
                QueryMsg::Approval {
                    token_id,
                    spender,
                    include_expired,
                } => cw721_base::QueryMsg::<Empty>::Approval {
                    token_id,
                    spender,
                    include_expired,
                },
                QueryMsg::Approvals {
                    token_id,
                    include_expired,
                } => cw721_base::QueryMsg::<Empty>::Approvals {
                    token_id,
                    include_expired,
                },
                QueryMsg::Operator {
                    owner,
                    operator,
                    include_expired,
                } => cw721_base::QueryMsg::<Empty>::Operator {
                    owner,
                    operator,
                    include_expired,
                },
                QueryMsg::AllOperators {
                    owner,
                    include_expired,
                    start_after,
                    limit,
                } => cw721_base::QueryMsg::<Empty>::AllOperators {
                    owner,
                    include_expired,
                    start_after,
                    limit,
                },
                QueryMsg::NumTokens {} => cw721_base::QueryMsg::<Empty>::NumTokens {},
                QueryMsg::ContractInfo {} => cw721_base::QueryMsg::<Empty>::ContractInfo {},
                QueryMsg::NftInfo { token_id } => {
                    cw721_base::QueryMsg::<Empty>::NftInfo { token_id }
                }
                QueryMsg::AllNftInfo {
                    token_id,
                    include_expired,
                } => cw721_base::QueryMsg::<Empty>::AllNftInfo {
                    token_id,
                    include_expired,
                },
                QueryMsg::Tokens {
                    owner,
                    start_after,
                    limit,
                } => cw721_base::QueryMsg::<Empty>::Tokens {
                    owner,
                    start_after,
                    limit,
                },
                QueryMsg::AllTokens { start_after, limit } => {
                    cw721_base::QueryMsg::<Empty>::AllTokens { start_after, limit }
                }
                _ => unreachable!("All other cases must be handled above"),
            };
            cw721_query(deps, env, cw721_msg)
        }
    }
}

fn query_tile_info(deps: Deps, token_id: String) -> StdResult<TileInfoResponse> {
    let metadata = TILE_METADATA.load(deps.storage, &token_id)?;
    let owner = deps.api.addr_validate("owner")?.to_string(); // TODO: Get actual owner

    Ok(TileInfoResponse { owner, metadata })
}

fn query_tile_at_position(deps: Deps, position: Position) -> StdResult<TileAtPositionResponse> {
    let token_ids = TILE_METADATA
        .idx
        .position
        .prefix((position.x, position.y))
        .keys(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(TileAtPositionResponse {
        token_id: token_ids.first().cloned(),
    })
}

fn query_enable_updatable(deps: Deps) -> StdResult<EnableUpdatableResponse> {
    let enabled = ENABLE_UPDATABLE.load(deps.storage)?;
    Ok(EnableUpdatableResponse { enabled })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
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

    fn setup_contract() -> (
        OwnedDeps<MockStorage, MockApi, CustomMockQuerier, Empty>,
        Env,
    ) {
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
        let res: TileInfoResponse =
            from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
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
        execute(deps.as_mut(), env.clone(), mock_info(MINTER, &[]), mint_msg).unwrap();

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
        let res: TileInfoResponse =
            from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
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
        assert_eq!(
            err.to_string(),
            ContractError::TokenMetadataFrozen {}.to_string()
        );
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
        execute(deps.as_mut(), env.clone(), mock_info(MINTER, &[]), mint_msg).unwrap();

        // Query existing position
        let query_msg = QueryMsg::TileAtPosition {
            position: Position { x: 2, y: 3 },
        };
        let res: TileAtPositionResponse =
            from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.token_id, Some(token_id.to_string()));

        // Query empty position
        let query_msg = QueryMsg::TileAtPosition {
            position: Position { x: 5, y: 5 },
        };
        let res: TileAtPositionResponse =
            from_binary(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.token_id, None);
    }
}
