use cosmwasm_std::{
    coin, entry_point, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, Event, MessageInfo,
    Response, StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::error::ContractError;
use crate::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, MintCountResponse, MintPositionResponse,
    MintPriceResponse, MintablePositionsResponse, QueryMsg,
};
use crate::state::{
    get_next_position, validate_position, Config, MintPosition, CONFIG, NEXT_POSITION,
    POSITION_TOKENS, TOTAL_MINTED,
};
use mosaic_tile_nft::msg::ExecuteMsg as NFTExecuteMsg;
use mosaic_tile_nft::state::{Color, Position};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:mosaic_vending_minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        mosaic_nft_address: deps.api.addr_validate(&msg.mosaic_nft_address)?,
        payment_address: deps.api.addr_validate(&msg.payment_address)?,
        unit_price: msg.unit_price,
        max_batch_size: msg.max_batch_size,
        random_minting_enabled: msg.random_minting_enabled,
        position_minting_enabled: msg.position_minting_enabled,
    };

    CONFIG.save(deps.storage, &config)?;
    TOTAL_MINTED.save(deps.storage, &0u32)?;
    NEXT_POSITION.save(deps.storage, &Position { x: 0, y: 0 })?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintRandom { color } => execute_mint_random(deps, env, info, color),
        ExecuteMsg::MintPosition { position, color } => {
            execute_mint_position(deps, env, info, position, color)
        }
        ExecuteMsg::BatchMintRandom { count, colors } => {
            execute_batch_mint_random(deps, env, info, count, colors)
        }
        ExecuteMsg::BatchMintPositions { mints } => {
            execute_batch_mint_positions(deps, env, info, mints)
        }
        ExecuteMsg::UpdateConfig {
            mosaic_nft_address,
            payment_address,
            unit_price,
            max_batch_size,
            random_minting_enabled,
            position_minting_enabled,
        } => execute_update_config(
            deps,
            info,
            mosaic_nft_address,
            payment_address,
            unit_price,
            max_batch_size,
            random_minting_enabled,
            position_minting_enabled,
        ),
    }
}

pub fn execute_mint_random(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    color: Color,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !config.random_minting_enabled {
        return Err(ContractError::RandomMintingDisabled {});
    }

    // Verify payment
    let payment = cw_utils::must_pay(&info, "ustars")?;
    if payment < config.unit_price {
        return Err(ContractError::InsufficientPayment {
            required: config.unit_price.u128(),
            sent: payment.u128(),
        });
    }

    // Find next available position
    let mut current_pos = NEXT_POSITION.load(deps.storage)?;
    while POSITION_TOKENS.has(deps.storage, (current_pos.x, current_pos.y)) {
        current_pos =
            get_next_position(current_pos).ok_or(ContractError::NoAvailablePositions {})?;
    }

    // Generate token ID
    let total_minted = TOTAL_MINTED.load(deps.storage)?;
    let token_id = format!("tile_{}", total_minted + 1);

    // Save position and update counters
    POSITION_TOKENS.save(
        deps.storage,
        (current_pos.x, current_pos.y),
        &Some(token_id.clone()),
    )?;
    TOTAL_MINTED.save(deps.storage, &(total_minted + 1))?;
    NEXT_POSITION.save(deps.storage, &current_pos)?;

    // Create mint message
    let mint_msg = NFTExecuteMsg::MintTile {
        token_id: token_id.clone(),
        owner: info.sender.to_string(),
        position: current_pos.clone(),
        color: color.clone(),
    };

    let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.mosaic_nft_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_random")
        .add_attribute("token_id", token_id)
        .add_attribute("position_x", current_pos.x.to_string())
        .add_attribute("position_y", current_pos.y.to_string()))
}

pub fn execute_mint_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position: Position,
    color: Color,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !config.position_minting_enabled {
        return Err(ContractError::PositionMintingDisabled {});
    }

    // Validate position
    if !validate_position(&position) {
        return Err(ContractError::InvalidPosition {
            x: position.x,
            y: position.y,
        });
    }

    // Check if position is available
    if POSITION_TOKENS.has(deps.storage, (position.x, position.y)) {
        return Err(ContractError::PositionTaken {
            x: position.x,
            y: position.y,
        });
    }

    // Verify payment
    let payment = cw_utils::must_pay(&info, "ustars")?;
    if payment < config.unit_price {
        return Err(ContractError::InsufficientPayment {
            required: config.unit_price.u128(),
            sent: payment.u128(),
        });
    }

    // Generate token ID
    let total_minted = TOTAL_MINTED.load(deps.storage)?;
    let token_id = format!("tile_{}", total_minted + 1);

    // Save position and update counter
    POSITION_TOKENS.save(
        deps.storage,
        (position.x, position.y),
        &Some(token_id.clone()),
    )?;
    TOTAL_MINTED.save(deps.storage, &(total_minted + 1))?;

    // Create mint message
    let mint_msg = NFTExecuteMsg::MintTile {
        token_id: token_id.clone(),
        owner: info.sender.to_string(),
        position: position.clone(),
        color: color.clone(),
    };

    let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.mosaic_nft_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_position")
        .add_attribute("token_id", token_id)
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string()))
}

pub fn execute_batch_mint_random(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    count: u32,
    colors: Vec<Color>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !config.random_minting_enabled {
        return Err(ContractError::RandomMintingDisabled {});
    }

    // Validate batch size
    if count > config.max_batch_size {
        return Err(ContractError::BatchSizeExceeded {});
    }
    if count as usize != colors.len() {
        return Err(ContractError::ColorCountMismatch {});
    }

    // Verify total payment
    let total_price = config.unit_price * Uint128::from(count);
    let payment = cw_utils::must_pay(&info, "ustars")?;
    if payment < total_price {
        return Err(ContractError::InsufficientPayment {
            required: total_price.u128(),
            sent: payment.u128(),
        });
    }

    let mut messages = Vec::with_capacity(count as usize);
    let mut current_pos = NEXT_POSITION.load(deps.storage)?;
    let mut total_minted = TOTAL_MINTED.load(deps.storage)?;

    for (i, color) in colors.into_iter().enumerate() {
        // Find next available position
        while POSITION_TOKENS.has(deps.storage, (current_pos.x, current_pos.y)) {
            current_pos =
                get_next_position(current_pos).ok_or(ContractError::NoAvailablePositions {})?;
        }

        // Generate token ID and save position
        let token_id = format!("tile_{}", total_minted + 1);
        POSITION_TOKENS.save(
            deps.storage,
            (current_pos.x, current_pos.y),
            &Some(token_id.clone()),
        )?;

        // Create mint message
        let mint_msg = NFTExecuteMsg::MintTile {
            token_id,
            owner: info.sender.to_string(),
            position: current_pos.clone(),
            color,
        };

        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.mosaic_nft_address.to_string(),
            msg: to_binary(&mint_msg)?,
            funds: vec![],
        }));

        total_minted += 1;
        current_pos =
            get_next_position(current_pos).ok_or(ContractError::NoAvailablePositions {})?;
    }

    // Update state
    TOTAL_MINTED.save(deps.storage, &total_minted)?;
    NEXT_POSITION.save(deps.storage, &current_pos)?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "batch_mint_random")
        .add_attribute("count", count.to_string()))
}

pub fn execute_batch_mint_positions(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mints: Vec<(Position, Color)>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !config.position_minting_enabled {
        return Err(ContractError::PositionMintingDisabled {});
    }

    // Validate batch size
    if mints.len() as u32 > config.max_batch_size {
        return Err(ContractError::BatchSizeExceeded {});
    }

    // Verify total payment
    let total_price = config.unit_price * Uint128::from(mints.len() as u32);
    let payment = cw_utils::must_pay(&info, "ustars")?;
    if payment < total_price {
        return Err(ContractError::InsufficientPayment {
            required: total_price.u128(),
            sent: payment.u128(),
        });
    }

    let mut messages = Vec::with_capacity(mints.len());
    let mut total_minted = TOTAL_MINTED.load(deps.storage)?;

    for (position, color) in mints {
        // Validate position
        if !validate_position(&position) {
            return Err(ContractError::InvalidPosition {
                x: position.x,
                y: position.y,
            });
        }

        // Check if position is available
        if POSITION_TOKENS.has(deps.storage, (position.x, position.y)) {
            return Err(ContractError::PositionTaken {
                x: position.x,
                y: position.y,
            });
        }

        // Generate token ID and save position
        let token_id = format!("tile_{}", total_minted + 1);
        POSITION_TOKENS.save(
            deps.storage,
            (position.x, position.y),
            &Some(token_id.clone()),
        )?;

        // Create mint message
        let mint_msg = NFTExecuteMsg::MintTile {
            token_id,
            owner: info.sender.to_string(),
            position: position.clone(),
            color,
        };

        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.mosaic_nft_address.to_string(),
            msg: to_binary(&mint_msg)?,
            funds: vec![],
        }));

        total_minted += 1;
    }

    // Update total minted
    TOTAL_MINTED.save(deps.storage, &total_minted)?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "batch_mint_positions")
        .add_attribute("count", mints.len().to_string()))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    mosaic_nft_address: Option<String>,
    payment_address: Option<String>,
    unit_price: Option<Uint128>,
    max_batch_size: Option<u32>,
    random_minting_enabled: Option<bool>,
    position_minting_enabled: Option<bool>,
) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        if info.sender != config.payment_address {
            return Err(ContractError::Unauthorized {});
        }

        if let Some(addr) = mosaic_nft_address {
            config.mosaic_nft_address = deps.api.addr_validate(&addr)?;
        }
        if let Some(addr) = payment_address {
            config.payment_address = deps.api.addr_validate(&addr)?;
        }
        if let Some(price) = unit_price {
            config.unit_price = price;
        }
        if let Some(size) = max_batch_size {
            config.max_batch_size = size;
        }
        if let Some(enabled) = random_minting_enabled {
            config.random_minting_enabled = enabled;
        }
        if let Some(enabled) = position_minting_enabled {
            config.position_minting_enabled = enabled;
        }

        Ok(config)
    })?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::MintPosition { position } => to_binary(&query_mint_position(deps, position)?),
        QueryMsg::MintCount {} => to_binary(&query_mint_count(deps)?),
        QueryMsg::MintPrice { count } => to_binary(&query_mint_price(deps, count)?),
        QueryMsg::MintablePositions { start_after, limit } => {
            to_binary(&query_mintable_positions(deps, start_after, limit)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        mosaic_nft_address: config.mosaic_nft_address,
        payment_address: config.payment_address,
        unit_price: config.unit_price,
        max_batch_size: config.max_batch_size,
        random_minting_enabled: config.random_minting_enabled,
        position_minting_enabled: config.position_minting_enabled,
    })
}

fn query_mint_position(deps: Deps, position: Position) -> StdResult<MintPositionResponse> {
    let token_id = POSITION_TOKENS.may_load(deps.storage, (position.x, position.y))?;
    Ok(MintPositionResponse {
        position,
        is_minted: token_id.is_some(),
        token_id,
    })
}

fn query_mint_count(deps: Deps) -> StdResult<MintCountResponse> {
    let total_minted = TOTAL_MINTED.load(deps.storage)?;
    Ok(MintCountResponse { total_minted })
}

fn query_mint_price(deps: Deps, count: u32) -> StdResult<MintPriceResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(MintPriceResponse {
        price: config.unit_price * Uint128::from(count),
    })
}

fn query_mintable_positions(
    deps: Deps,
    start_after: Option<Position>,
    limit: Option<u32>,
) -> StdResult<MintablePositionsResponse> {
    let limit = limit.unwrap_or(10) as usize;
    let start = start_after.unwrap_or(Position { x: 0, y: 0 });
    let mut positions = Vec::with_capacity(limit);
    let mut current = start;

    while positions.len() < limit {
        if !POSITION_TOKENS.has(deps.storage, (current.x, current.y)) {
            positions.push(current.clone());
        }

        if let Some(next) = get_next_position(current) {
            current = next;
        } else {
            break;
        }
    }

    Ok(MintablePositionsResponse { positions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr};

    const OWNER: &str = "owner";
    const NFT_CONTRACT: &str = "nft_contract";
    const UNIT_PRICE: u128 = 1_000_000;

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg {
            mosaic_nft_address: NFT_CONTRACT.to_string(),
            payment_address: OWNER.to_string(),
            unit_price: Uint128::from(UNIT_PRICE),
            max_batch_size: 10,
            random_minting_enabled: true,
            position_minting_enabled: true,
        };
        let info = mock_info(OWNER, &[]);
        let env = mock_env();
        instantiate(deps, env, info, msg).unwrap();
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        assert_eq!(config.mosaic_nft_address, Addr::unchecked(NFT_CONTRACT));
        assert_eq!(config.payment_address, Addr::unchecked(OWNER));
        assert_eq!(config.unit_price, Uint128::from(UNIT_PRICE));
        assert_eq!(config.max_batch_size, 10);
        assert!(config.random_minting_enabled);
        assert!(config.position_minting_enabled);

        let total_minted = TOTAL_MINTED.load(deps.as_ref().storage).unwrap();
        assert_eq!(total_minted, 0);

        let next_pos = NEXT_POSITION.load(deps.as_ref().storage).unwrap();
        assert_eq!(next_pos.x, 0);
        assert_eq!(next_pos.y, 0);
    }

    #[test]
    fn mint_random_tile() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // Test insufficient payment
        let info = mock_info("buyer", &coins(UNIT_PRICE - 1, "ustars"));
        let msg = ExecuteMsg::MintRandom {
            color: Color { r: 255, g: 0, b: 0 },
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert!(matches!(err, ContractError::InsufficientPayment { .. }));

        // Test successful mint
        let info = mock_info("buyer", &coins(UNIT_PRICE, "ustars"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(1, res.messages.len());
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "action" && attr.value == "mint_random"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "token_id" && attr.value == "tile_1"));

        // Verify state updates
        let total_minted = TOTAL_MINTED.load(deps.as_ref().storage).unwrap();
        assert_eq!(total_minted, 1);

        let next_pos = NEXT_POSITION.load(deps.as_ref().storage).unwrap();
        assert_eq!(next_pos.x, 1);
        assert_eq!(next_pos.y, 0);

        // Test when random minting is disabled
        CONFIG
            .update(deps.as_mut().storage, |mut config| -> StdResult<_> {
                config.random_minting_enabled = false;
                Ok(config)
            })
            .unwrap();

        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::RandomMintingDisabled {}));
    }

    #[test]
    fn mint_position_tile() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 5, y: 5 };
        let color = Color { r: 0, g: 255, b: 0 };

        // Test insufficient payment
        let info = mock_info("buyer", &coins(UNIT_PRICE - 1, "ustars"));
        let msg = ExecuteMsg::MintPosition {
            position: position.clone(),
            color: color.clone(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert!(matches!(err, ContractError::InsufficientPayment { .. }));

        // Test successful mint
        let info = mock_info("buyer", &coins(UNIT_PRICE, "ustars"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(1, res.messages.len());
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "action" && attr.value == "mint_position"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "token_id" && attr.value == "tile_1"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "position_x" && attr.value == "5"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "position_y" && attr.value == "5"));

        // Test minting same position again
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert!(matches!(err, ContractError::PositionTaken { .. }));

        // Test when position minting is disabled
        CONFIG
            .update(deps.as_mut().storage, |mut config| -> StdResult<_> {
                config.position_minting_enabled = false;
                Ok(config)
            })
            .unwrap();

        let msg = ExecuteMsg::MintPosition {
            position: Position { x: 6, y: 6 },
            color,
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::PositionMintingDisabled {}));
    }

    #[test]
    fn batch_mint_random() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let colors = vec![
            Color { r: 255, g: 0, b: 0 },
            Color { r: 0, g: 255, b: 0 },
            Color { r: 0, g: 0, b: 255 },
        ];

        // Test insufficient payment
        let info = mock_info("buyer", &coins(UNIT_PRICE * 2, "ustars"));
        let msg = ExecuteMsg::BatchMintRandom {
            count: 3,
            colors: colors.clone(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert!(matches!(err, ContractError::InsufficientPayment { .. }));

        // Test successful batch mint
        let info = mock_info("buyer", &coins(UNIT_PRICE * 3, "ustars"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(3, res.messages.len());
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "action" && attr.value == "batch_mint_random"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "count" && attr.value == "3"));

        // Verify state updates
        let total_minted = TOTAL_MINTED.load(deps.as_ref().storage).unwrap();
        assert_eq!(total_minted, 3);

        // Test batch size exceeded
        let msg = ExecuteMsg::BatchMintRandom {
            count: 11,
            colors: vec![Color { r: 0, g: 0, b: 0 }; 11],
        };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert!(matches!(err, ContractError::BatchSizeExceeded {}));

        // Test color count mismatch
        let msg = ExecuteMsg::BatchMintRandom {
            count: 3,
            colors: vec![Color { r: 0, g: 0, b: 0 }; 2],
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::ColorCountMismatch {}));
    }

    #[test]
    fn batch_mint_positions() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let mints = vec![
            (Position { x: 1, y: 1 }, Color { r: 255, g: 0, b: 0 }),
            (Position { x: 2, y: 2 }, Color { r: 0, g: 255, b: 0 }),
            (Position { x: 3, y: 3 }, Color { r: 0, g: 0, b: 255 }),
        ];

        // Test insufficient payment
        let info = mock_info("buyer", &coins(UNIT_PRICE * 2, "ustars"));
        let msg = ExecuteMsg::BatchMintPositions {
            mints: mints.clone(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert!(matches!(err, ContractError::InsufficientPayment { .. }));

        // Test successful batch mint
        let info = mock_info("buyer", &coins(UNIT_PRICE * 3, "ustars"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(3, res.messages.len());
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "action" && attr.value == "batch_mint_positions"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "count" && attr.value == "3"));

        // Test minting same positions again
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert!(matches!(err, ContractError::PositionTaken { .. }));

        // Test batch size exceeded
        let large_mints = (0..11)
            .map(|i| {
                (
                    Position {
                        x: i + 10,
                        y: i + 10,
                    },
                    Color { r: 0, g: 0, b: 0 },
                )
            })
            .collect::<Vec<_>>();
        let msg = ExecuteMsg::BatchMintPositions { mints: large_mints };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::BatchSizeExceeded {}));
    }

    #[test]
    fn update_config() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // Test unauthorized update
        let info = mock_info("anyone", &[]);
        let msg = ExecuteMsg::UpdateConfig {
            mosaic_nft_address: None,
            payment_address: None,
            unit_price: Some(Uint128::from(2_000_000u128)),
            max_batch_size: None,
            random_minting_enabled: None,
            position_minting_enabled: None,
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized {}));

        // Test successful update
        let info = mock_info(OWNER, &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "action" && attr.value == "update_config"));

        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        assert_eq!(config.unit_price, Uint128::from(2_000_000u128));
    }

    #[test]
    fn query_tests() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // Test config query
        let config: ConfigResponse =
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap();
        assert_eq!(config.unit_price, Uint128::from(UNIT_PRICE));

        // Test mint position query
        let pos = Position { x: 5, y: 5 };
        let pos_info: MintPositionResponse = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::MintPosition {
                    position: pos.clone(),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert!(!pos_info.is_minted);
        assert_eq!(pos_info.token_id, None);

        // Test mint count query
        let count: MintCountResponse =
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::MintCount {}).unwrap())
                .unwrap();
        assert_eq!(count.total_minted, 0);

        // Test mint price query
        let price: MintPriceResponse = from_binary(
            &query(deps.as_ref(), mock_env(), QueryMsg::MintPrice { count: 3 }).unwrap(),
        )
        .unwrap();
        assert_eq!(price.price, Uint128::from(UNIT_PRICE * 3));

        // Test mintable positions query
        let positions: MintablePositionsResponse = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::MintablePositions {
                    start_after: None,
                    limit: Some(5),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(positions.positions.len(), 5);
        assert_eq!(positions.positions[0], Position { x: 0, y: 0 });
    }
}
