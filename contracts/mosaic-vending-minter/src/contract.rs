use crate::error::ContractError;
use crate::msg::{
    BatchMintPositionsResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, MintCountResponse,
    MintPositionResponse, MintPriceResponse, MintablePositionsResponse, QueryMsg,
};
use crate::state::{Config, MintPosition, CONFIG, NEXT_POSITION, POSITION_TOKENS, TOTAL_MINTED, get_next_position, validate_position};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg, CosmosMsg, BankMsg, coins,
};
use cw2::set_contract_version;
use mosaic_tile_nft::msg::ExecuteMsg as NFTExecuteMsg;
use mosaic_tile_nft::state::{Position, Color};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:mosaic-vending-minter";
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

    // Initialize starting position for random minting
    NEXT_POSITION.save(deps.storage, &Position { x: 0, y: 0 })?;
    TOTAL_MINTED.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
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

    // Validate payment
    validate_payment(&info, config.unit_price)?;

    // Get next available position
    let position = find_random_position(deps.as_ref(), &env)?;

    // Mint the tile
    let res = mint_tile(deps, env, info, config, position, color)?;
    Ok(res)
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

    // Check if position is already taken
    if POSITION_TOKENS.has(deps.storage, (position.x, position.y)) {
        return Err(ContractError::PositionTaken {
            x: position.x,
            y: position.y,
        });
    }

    // Validate payment
    validate_payment(&info, config.unit_price)?;

    // Mint the tile
    let res = mint_tile(deps, env, info, config, position, color)?;
    Ok(res)
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

    if count > config.max_batch_size {
        return Err(ContractError::BatchSizeExceeded {});
    }

    if colors.len() != count as usize {
        return Err(ContractError::ColorCountMismatch {});
    }

    // Validate total payment
    let total_price = config.unit_price * Uint128::from(count);
    validate_payment(&info, total_price)?;

    let mut messages = vec![];
    let mut positions = vec![];

    // Generate random positions and create mint messages
    for color in colors {
        let position = find_random_position(deps.as_ref(), &env)?;
        positions.push(position.clone());
        
        let mint_msg = create_mint_message(
            &config.mosaic_nft_address,
            position.clone(),
            color,
        )?;
        messages.push(mint_msg);

        // Update position tracking
        POSITION_TOKENS.save(deps.storage, (position.x, position.y), &None)?;
    }

    // Update total minted count
    let current_total = TOTAL_MINTED.load(deps.storage)?;
    TOTAL_MINTED.save(deps.storage, &(current_total + count))?;

    // Create payment message
    let payment_msg = BankMsg::Send {
        to_address: config.payment_address.to_string(),
        amount: coins(total_price.u128(), "ustars"),
    };

    Ok(Response::new()
        .add_messages(messages)
        .add_message(payment_msg)
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

    if mints.len() as u32 > config.max_batch_size {
        return Err(ContractError::BatchSizeExceeded {});
    }

    // Validate total payment
    let total_price = config.unit_price * Uint128::from(mints.len() as u32);
    validate_payment(&info, total_price)?;

    let mut messages = vec![];

    // Validate positions and create mint messages
    for (position, color) in mints.iter() {
        if !validate_position(position) {
            return Err(ContractError::InvalidPosition {
                x: position.x,
                y: position.y,
            });
        }

        if POSITION_TOKENS.has(deps.storage, (position.x, position.y)) {
            return Err(ContractError::PositionTaken {
                x: position.x,
                y: position.y,
            });
        }

        let mint_msg = create_mint_message(
            &config.mosaic_nft_address,
            position.clone(),
            color.clone(),
        )?;
        messages.push(mint_msg);

        // Update position tracking
        POSITION_TOKENS.save(deps.storage, (position.x, position.y), &None)?;
    }

    // Update total minted count
    let current_total = TOTAL_MINTED.load(deps.storage)?;
    TOTAL_MINTED.save(deps.storage, &(current_total + mints.len() as u32))?;

    // Create payment message
    let payment_msg = BankMsg::Send {
        to_address: config.payment_address.to_string(),
        amount: coins(total_price.u128(), "ustars"),
    };

    Ok(Response::new()
        .add_messages(messages)
        .add_message(payment_msg)
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
    let mut config = CONFIG.load(deps.storage)?;

    // Only the payment address (contract owner) can update config
    if info.sender != config.payment_address {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(address) = mosaic_nft_address {
        config.mosaic_nft_address = deps.api.addr_validate(&address)?;
    }
    if let Some(address) = payment_address {
        config.payment_address = deps.api.addr_validate(&address)?;
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

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

// Helper functions

fn validate_payment(info: &MessageInfo, required_price: Uint128) -> Result<(), ContractError> {
    let payment = info
        .funds
        .iter()
        .find(|c| c.denom == "ustars")
        .map(|c| c.amount)
        .unwrap_or_default();

    if payment != required_price {
        return Err(ContractError::InvalidPayment {});
    }

    Ok(())
}

fn find_random_position(deps: Deps, env: &Env) -> Result<Position, ContractError> {
    // Create a deterministic RNG based on block time and height
    let seed = (env.block.time.nanos() as u64)
        .wrapping_mul(env.block.height as u64);
    let mut rng = StdRng::seed_from_u64(seed);

    // Try up to 100 times to find an available position
    for _ in 0..100 {
        let x = rng.gen_range(0..=MAX_POSITION);
        let y = rng.gen_range(0..=MAX_POSITION);
        let position = Position { x, y };

        if !POSITION_TOKENS.has(deps.storage, (x, y)) {
            return Ok(position);
        }
    }

    // If we couldn't find a random position, fall back to sequential search
    let current = NEXT_POSITION.load(deps.storage)?;
    let mut next = current;

    while POSITION_TOKENS.has(deps.storage, (next.x, next.y)) {
        next = get_next_position(next)
            .ok_or(ContractError::NoAvailablePositions {})?;
    }

    NEXT_POSITION.save(deps.storage, &next)?;
    Ok(next)
}

fn create_mint_message(
    nft_contract: &str,
    position: Position,
    color: Color,
) -> StdResult<CosmosMsg> {
    let token_id = format!("tile_{}-{}", position.x, position.y);
    
    let msg = NFTExecuteMsg::MintTile {
        token_id: token_id.clone(),
        owner: info.sender.to_string(),
        position,
        color,
    };

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: nft_contract.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    }))
}

fn mint_tile(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    config: Config,
    position: Position,
    color: Color,
) -> Result<Response, ContractError> {
    let token_id = format!("tile_{}-{}", position.x, position.y);

    // Create mint message
    let mint_msg = create_mint_message(
        &config.mosaic_nft_address.to_string(),
        position.clone(),
        color,
    )?;

    // Update position tracking
    POSITION_TOKENS.save(deps.storage, (position.x, position.y), &Some(token_id.clone()))?;

    // Update total minted count
    let current_total = TOTAL_MINTED.load(deps.storage)?;
    TOTAL_MINTED.save(deps.storage, &(current_total + 1))?;

    // Create payment message
    let payment_msg = BankMsg::Send {
        to_address: config.payment_address.to_string(),
        amount: coins(config.unit_price.u128(), "ustars"),
    };

    Ok(Response::new()
        .add_message(mint_msg)
        .add_message(payment_msg)
        .add_attribute("action", "mint_tile")
        .add_attribute("token_id", token_id)
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string()))
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
    let price = config.unit_price * Uint128::from(count);
    Ok(MintPriceResponse { price })
}

fn query_mintable_positions(
    deps: Deps,
    start_after: Option<Position>,
    limit: Option<u32>,
) -> StdResult<MintablePositionsResponse> {
    let limit = limit.unwrap_or(10) as usize;
    let start = start_after.unwrap_or(Position { x: 0, y: 0 });
    let mut positions = vec![];

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
    }

    #[test]
    fn mint_random_tile() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let info = mock_info(
            "buyer",
            &coins(UNIT_PRICE, "ustars"),
        );
        let msg = ExecuteMsg::MintRandom {
            color: Color { r: 255, g: 0, b: 0 },
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(2, res.messages.len()); // Mint message and payment message
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "mint_tile"));
    }

    #[test]
    fn mint_specific_position() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let info = mock_info(
            "buyer",
            &coins(UNIT_PRICE, "ustars"),
        );
        let msg = ExecuteMsg::MintPosition {
            position: Position { x: 5, y: 5 },
            color: Color { r: 0, g: 255, b: 0 },
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(2, res.messages.len());
        assert!(res.attributes.iter().any(|attr| attr.key == "position_x" && attr.value == "5"));

        // Try to mint same position again
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(
            err,
            ContractError::PositionTaken { x: 5, y: 5 }
        );
    }

    #[test]
    fn batch_mint_random() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let info = mock_info(
            "buyer",
            &coins(UNIT_PRICE * 3, "ustars"),
        );
        let msg = ExecuteMsg::BatchMintRandom {
            count: 3,
            colors: vec![
                Color { r: 255, g: 0, b: 0 },
                Color { r: 0, g: 255, b: 0 },
                Color { r: 0, g: 0, b: 255 },
            ],
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(4, res.messages.len()); // 3 mint messages + 1 payment message
        assert!(res.attributes.iter().any(|attr| attr.key == "count" && attr.value == "3"));
    }

    #[test]
    fn update_config() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // Only owner can update config
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
        assert_eq!(err, ContractError::Unauthorized {});

        // Owner can update config
        let info = mock_info(OWNER, &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "update_config"));

        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        assert_eq!(config.unit_price, Uint128::from(2_000_000u128));
    }
} 