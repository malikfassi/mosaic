use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, TilePermissionsResponse,
    ColorHistoryResponse, UserStatisticsResponse, TotalFeesResponse, CanChangeColorResponse,
};
use crate::state::{
    Config, UserStatistics, TilePermissions, ColorChangeEvent,
    CONFIG, USER_STATS, TILE_PERMISSIONS, COLOR_HISTORY, TOTAL_FEES,
    can_change_color, check_rate_limit,
};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128, WasmMsg, CosmosMsg, BankMsg, coins, Addr,
};
use cw2::set_contract_version;
use mosaic_tile_nft::msg::ExecuteMsg as NFTExecuteMsg;
use mosaic_tile_nft::state::{Position, Color};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:tile-coloring";
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
        nft_contract: deps.api.addr_validate(&msg.nft_contract)?,
        admin: deps.api.addr_validate(&msg.admin)?,
        color_change_fee: msg.color_change_fee,
        rate_limit: msg.rate_limit,
        rate_limit_window: msg.rate_limit_window,
        requires_payment: msg.requires_payment,
        rate_limiting_enabled: msg.rate_limiting_enabled,
    };
    CONFIG.save(deps.storage, &config)?;

    // Initialize total fees
    TOTAL_FEES.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", msg.admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeColor { position, color } => {
            execute_change_color(deps, env, info, position, color)
        }
        ExecuteMsg::GrantPermission { position, editor, expires_at } => {
            execute_grant_permission(deps, env, info, position, editor, expires_at)
        }
        ExecuteMsg::RevokePermission { position, editor } => {
            execute_revoke_permission(deps, env, info, position, editor)
        }
        ExecuteMsg::SetPublicEditing { position, public_editing, public_change_fee } => {
            execute_set_public_editing(deps, env, info, position, public_editing, public_change_fee)
        }
        ExecuteMsg::UpdateConfig { 
            nft_contract, admin, color_change_fee, rate_limit,
            rate_limit_window, requires_payment, rate_limiting_enabled 
        } => execute_update_config(
            deps, info, nft_contract, admin, color_change_fee, rate_limit,
            rate_limit_window, requires_payment, rate_limiting_enabled,
        ),
        ExecuteMsg::WithdrawFees { amount } => execute_withdraw_fees(deps, info, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::TilePermissions { position } => {
            to_binary(&query_tile_permissions(deps, position)?)
        }
        QueryMsg::ColorHistory { position, start_after, limit } => {
            to_binary(&query_color_history(deps, position, start_after, limit)?)
        }
        QueryMsg::UserStatistics { address } => {
            to_binary(&query_user_statistics(deps, address)?)
        }
        QueryMsg::TotalFees {} => to_binary(&query_total_fees(deps)?),
        QueryMsg::CanChangeColor { position, editor } => {
            to_binary(&query_can_change_color(deps, env, position, editor)?)
        }
    }
} 

pub fn execute_change_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position: Position,
    color: Color,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Load or create tile permissions
    let permissions = TILE_PERMISSIONS
        .may_load(deps.storage, (position.x, position.y))?
        .unwrap_or_else(|| TilePermissions {
            owner: info.sender.clone(), // Will be updated with NFT owner
            allowed_editors: vec![],
            public_editing: false,
            permission_expiry: None,
            public_change_fee: None,
        });

    // Verify permission to change color
    if !can_change_color(&permissions, &info.sender, &env.block.time) {
        return Err(ContractError::ColorChangeNotAllowed {});
    }

    // Check rate limit
    let mut user_stats = USER_STATS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    
    if !check_rate_limit(&user_stats, &config, env.block.time) {
        let window_end = user_stats.current_window_start.unwrap()
            .plus_seconds(config.rate_limit_window);
        let remaining = window_end.seconds() - env.block.time.seconds();
        return Err(ContractError::RateLimitExceeded { seconds: remaining });
    }

    // Calculate and verify payment
    let required_fee = if permissions.owner == info.sender {
        Uint128::zero()
    } else {
        permissions.public_change_fee.unwrap_or(config.color_change_fee)
    };

    if config.requires_payment && required_fee > Uint128::zero() {
        let payment = info.funds
            .iter()
            .find(|c| c.denom == "ustars")
            .map(|c| c.amount)
            .unwrap_or_default();

        if payment < required_fee {
            return Err(ContractError::InsufficientPayment {
                required: required_fee.u128(),
                sent: payment.u128(),
            });
        }
    }

    // Update user statistics
    user_stats.total_color_changes += 1;
    user_stats.total_fees_paid += required_fee;
    user_stats.last_color_change = Some(env.block.time);
    user_stats.changes_in_window += 1;
    if user_stats.current_window_start.is_none() {
        user_stats.current_window_start = Some(env.block.time);
    }
    USER_STATS.save(deps.storage, &info.sender, &user_stats)?;

    // Update total fees
    if required_fee > Uint128::zero() {
        TOTAL_FEES.update(deps.storage, |fees| -> StdResult<_> {
            Ok(fees + required_fee)
        })?;
    }

    // Record color change event
    let event = ColorChangeEvent {
        editor: info.sender.clone(),
        from_color: color.clone(), // TODO: Get current color from NFT contract
        to_color: color.clone(),
        timestamp: env.block.time,
        fee_paid: Some(required_fee),
    };

    COLOR_HISTORY.update(
        deps.storage,
        (position.x, position.y),
        |history| -> StdResult<_> {
            let mut history = history.unwrap_or_default();
            history.push(event);
            Ok(history)
        },
    )?;

    // Create color update message for NFT contract
    let update_msg = NFTExecuteMsg::UpdateTileColor {
        token_id: format!("tile_{}-{}", position.x, position.y),
        color,
    };

    let update_color_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.nft_contract.to_string(),
        msg: to_binary(&update_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(update_color_msg)
        .add_attribute("action", "change_color")
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string())
        .add_attribute("editor", info.sender))
}

pub fn execute_grant_permission(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position: Position,
    editor: String,
    expires_at: Option<Timestamp>,
) -> Result<Response, ContractError> {
    let editor_addr = deps.api.addr_validate(&editor)?;
    
    TILE_PERMISSIONS.update(
        deps.storage,
        (position.x, position.y),
        |permissions| -> Result<_, ContractError> {
            let mut permissions = permissions.unwrap_or_else(|| TilePermissions {
                owner: info.sender.clone(),
                allowed_editors: vec![],
                public_editing: false,
                permission_expiry: None,
                public_change_fee: None,
            });

            // Only owner can grant permissions
            if info.sender != permissions.owner {
                return Err(ContractError::Unauthorized {});
            }

            // Check if editor already has permission
            if permissions.allowed_editors.contains(&editor_addr) {
                return Err(ContractError::PermissionAlreadyGranted {
                    address: editor.clone(),
                });
            }

            // Validate expiry time
            if let Some(expiry) = expires_at {
                if expiry <= env.block.time {
                    return Err(ContractError::InvalidExpiryTime {});
                }
            }

            permissions.allowed_editors.push(editor_addr.clone());
            permissions.permission_expiry = expires_at;
            Ok(permissions)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "grant_permission")
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string())
        .add_attribute("editor", editor))
}

pub fn execute_revoke_permission(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    position: Position,
    editor: String,
) -> Result<Response, ContractError> {
    let editor_addr = deps.api.addr_validate(&editor)?;
    
    TILE_PERMISSIONS.update(
        deps.storage,
        (position.x, position.y),
        |permissions| -> Result<_, ContractError> {
            let mut permissions = permissions.ok_or(ContractError::TileNotFound {})?;

            // Only owner can revoke permissions
            if info.sender != permissions.owner {
                return Err(ContractError::Unauthorized {});
            }

            // Remove editor from allowed list
            let pos = permissions.allowed_editors.iter()
                .position(|addr| addr == &editor_addr)
                .ok_or_else(|| ContractError::PermissionNotFound {
                    address: editor.clone(),
                })?;
            permissions.allowed_editors.remove(pos);

            Ok(permissions)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "revoke_permission")
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string())
        .add_attribute("editor", editor))
}

pub fn execute_set_public_editing(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    position: Position,
    public_editing: bool,
    public_change_fee: Option<Uint128>,
) -> Result<Response, ContractError> {
    TILE_PERMISSIONS.update(
        deps.storage,
        (position.x, position.y),
        |permissions| -> Result<_, ContractError> {
            let mut permissions = permissions.ok_or(ContractError::TileNotFound {})?;

            // Only owner can change public editing settings
            if info.sender != permissions.owner {
                return Err(ContractError::Unauthorized {});
            }

            permissions.public_editing = public_editing;
            permissions.public_change_fee = public_change_fee;

            Ok(permissions)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "set_public_editing")
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string())
        .add_attribute("public_editing", public_editing.to_string()))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract: Option<String>,
    admin: Option<String>,
    color_change_fee: Option<Uint128>,
    rate_limit: Option<u32>,
    rate_limit_window: Option<u64>,
    requires_payment: Option<bool>,
    rate_limiting_enabled: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only admin can update config
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(contract) = nft_contract {
        config.nft_contract = deps.api.addr_validate(&contract)?;
    }
    if let Some(new_admin) = admin {
        config.admin = deps.api.addr_validate(&new_admin)?;
    }
    if let Some(fee) = color_change_fee {
        config.color_change_fee = fee;
    }
    if let Some(limit) = rate_limit {
        config.rate_limit = limit;
    }
    if let Some(window) = rate_limit_window {
        config.rate_limit_window = window;
    }
    if let Some(payment) = requires_payment {
        config.requires_payment = payment;
    }
    if let Some(enabled) = rate_limiting_enabled {
        config.rate_limiting_enabled = enabled;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_withdraw_fees(
    deps: DepsMut,
    info: MessageInfo,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only admin can withdraw fees
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let total_fees = TOTAL_FEES.load(deps.storage)?;
    let withdraw_amount = amount.unwrap_or(total_fees);

    if withdraw_amount > total_fees {
        return Err(ContractError::InvalidFeeAmount {});
    }

    // Update total fees
    TOTAL_FEES.save(deps.storage, &(total_fees - withdraw_amount))?;

    // Create bank send message
    let bank_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(withdraw_amount.u128(), "ustars"),
    };

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw_fees")
        .add_attribute("amount", withdraw_amount))
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        nft_contract: config.nft_contract,
        admin: config.admin,
        color_change_fee: config.color_change_fee,
        rate_limit: config.rate_limit,
        rate_limit_window: config.rate_limit_window,
        requires_payment: config.requires_payment,
        rate_limiting_enabled: config.rate_limiting_enabled,
        total_tiles_modified: 0, // TODO: Track this in state
    })
}

fn query_tile_permissions(deps: Deps, position: Position) -> StdResult<TilePermissionsResponse> {
    let permissions = TILE_PERMISSIONS
        .may_load(deps.storage, (position.x, position.y))?
        .unwrap_or_else(|| TilePermissions {
            owner: Addr::unchecked(""), // Will be fetched from NFT contract
            allowed_editors: vec![],
            public_editing: false,
            permission_expiry: None,
            public_change_fee: None,
        });

    Ok(TilePermissionsResponse {
        position,
        permissions,
    })
}

fn query_color_history(
    deps: Deps,
    position: Position,
    start_after: Option<Timestamp>,
    limit: Option<u32>,
) -> StdResult<ColorHistoryResponse> {
    let history = COLOR_HISTORY
        .may_load(deps.storage, (position.x, position.y))?
        .unwrap_or_default();

    let limit = limit.unwrap_or(30) as usize;
    let filtered: Vec<ColorChangeEvent> = history
        .into_iter()
        .filter(|event| {
            if let Some(start) = start_after {
                event.timestamp > start
            } else {
                true
            }
        })
        .take(limit)
        .collect();

    Ok(ColorHistoryResponse {
        position,
        history: filtered,
    })
}

fn query_user_statistics(deps: Deps, address: String) -> StdResult<UserStatisticsResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let statistics = USER_STATS
        .may_load(deps.storage, &addr)?
        .unwrap_or_default();

    Ok(UserStatisticsResponse {
        address: addr,
        statistics,
    })
}

fn query_total_fees(deps: Deps) -> StdResult<TotalFeesResponse> {
    let total_fees = TOTAL_FEES.load(deps.storage)?;
    Ok(TotalFeesResponse { total_fees })
}

fn query_can_change_color(
    deps: Deps,
    env: Env,
    position: Position,
    editor: String,
) -> StdResult<CanChangeColorResponse> {
    let editor_addr = deps.api.addr_validate(&editor)?;
    let config = CONFIG.load(deps.storage)?;
    
    // Get permissions
    let permissions = TILE_PERMISSIONS
        .may_load(deps.storage, (position.x, position.y))?
        .unwrap_or_else(|| TilePermissions {
            owner: Addr::unchecked(""), // Will be fetched from NFT contract
            allowed_editors: vec![],
            public_editing: false,
            permission_expiry: None,
            public_change_fee: None,
        });

    // Check basic permission
    if !can_change_color(&permissions, &editor_addr, &env.block.time) {
        return Ok(CanChangeColorResponse {
            can_change: false,
            reason: Some("No permission to change color".to_string()),
            required_fee: None,
        });
    }

    // Check rate limit
    let user_stats = USER_STATS
        .may_load(deps.storage, &editor_addr)?
        .unwrap_or_default();
    
    if !check_rate_limit(&user_stats, &config, env.block.time) {
        return Ok(CanChangeColorResponse {
            can_change: false,
            reason: Some("Rate limit exceeded".to_string()),
            required_fee: None,
        });
    }

    // Calculate required fee
    let required_fee = if permissions.owner == editor_addr {
        None
    } else {
        Some(permissions.public_change_fee.unwrap_or(config.color_change_fee))
    };

    Ok(CanChangeColorResponse {
        can_change: true,
        reason: None,
        required_fee,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr};

    const ADMIN: &str = "admin";
    const NFT_CONTRACT: &str = "nft_contract";
    const USER1: &str = "user1";
    const USER2: &str = "user2";

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg {
            nft_contract: NFT_CONTRACT.to_string(),
            admin: ADMIN.to_string(),
            color_change_fee: Uint128::from(1000000u128),
            rate_limit: 10,
            rate_limit_window: 3600,
            requires_payment: true,
            rate_limiting_enabled: true,
        };
        let info = mock_info(ADMIN, &[]);
        let env = mock_env();
        instantiate(deps, env, info, msg).unwrap();
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        assert_eq!(config.nft_contract, Addr::unchecked(NFT_CONTRACT));
        assert_eq!(config.admin, Addr::unchecked(ADMIN));
        assert_eq!(config.color_change_fee, Uint128::from(1000000u128));
        assert_eq!(config.rate_limit, 10);
        assert!(config.requires_payment);
        assert!(config.rate_limiting_enabled);
    }

    #[test]
    fn test_change_color() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // Set up tile permissions
        let position = Position { x: 1, y: 1 };
        let permissions = TilePermissions {
            owner: Addr::unchecked(USER1),
            allowed_editors: vec![],
            public_editing: false,
            permission_expiry: None,
            public_change_fee: None,
        };
        TILE_PERMISSIONS
            .save(deps.as_mut().storage, (position.x, position.y), &permissions)
            .unwrap();

        // Owner can change color without payment
        let msg = ExecuteMsg::ChangeColor {
            position: position.clone(),
            color: Color { r: 255, g: 0, b: 0 },
        };
        let info = mock_info(USER1, &[]);
        let env = mock_env();
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap();
        assert_eq!(1, res.messages.len()); // NFT update message

        // Non-owner cannot change color
        let info = mock_info(USER2, &coins(1000000, "ustars"));
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::ColorChangeNotAllowed {});
    }

    #[test]
    fn test_permissions() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 1, y: 1 };

        // Grant permission
        let msg = ExecuteMsg::GrantPermission {
            position: position.clone(),
            editor: USER2.to_string(),
            expires_at: None,
        };
        let info = mock_info(USER1, &[]);
        let env = mock_env();
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Check if permission was granted
        let res = query_tile_permissions(deps.as_ref(), position.clone()).unwrap();
        assert!(res.permissions.allowed_editors.contains(&Addr::unchecked(USER2)));

        // Revoke permission
        let msg = ExecuteMsg::RevokePermission {
            position,
            editor: USER2.to_string(),
        };
        let info = mock_info(USER1, &[]);
        execute(deps.as_mut(), env, info, msg).unwrap();

        // Check if permission was revoked
        let res = query_tile_permissions(deps.as_ref(), position).unwrap();
        assert!(!res.permissions.allowed_editors.contains(&Addr::unchecked(USER2)));
    }

    #[test]
    fn test_public_editing() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 1, y: 1 };
        let fee = Uint128::from(500000u128);

        // Enable public editing
        let msg = ExecuteMsg::SetPublicEditing {
            position: position.clone(),
            public_editing: true,
            public_change_fee: Some(fee),
        };
        let info = mock_info(USER1, &[]);
        let env = mock_env();
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Check if public editing is enabled
        let res = query_tile_permissions(deps.as_ref(), position.clone()).unwrap();
        assert!(res.permissions.public_editing);
        assert_eq!(res.permissions.public_change_fee, Some(fee));

        // Anyone can change color with payment
        let msg = ExecuteMsg::ChangeColor {
            position,
            color: Color { r: 0, g: 255, b: 0 },
        };
        let info = mock_info(USER2, &coins(500000, "ustars"));
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn test_rate_limiting() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 1, y: 1 };
        let mut env = mock_env();

        // Make multiple color changes
        for i in 0..10 {
            let msg = ExecuteMsg::ChangeColor {
                position: position.clone(),
                color: Color { r: i as u8, g: 0, b: 0 },
            };
            let info = mock_info(USER1, &[]);
            execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        }

        // Next change should fail due to rate limit
        let msg = ExecuteMsg::ChangeColor {
            position,
            color: Color { r: 255, g: 0, b: 0 },
        };
        let info = mock_info(USER1, &[]);
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        match err {
            ContractError::RateLimitExceeded { seconds: _ } => {}
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }
}