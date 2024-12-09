use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Addr, Uint128, Event, CosmosMsg, BankMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    Config, UserStatistics, TilePermissions, ColorChangeEvent,
    CONFIG, USER_STATS, TILE_PERMISSIONS, TOTAL_FEES,
    can_change_color, check_rate_limit,
};
use mosaic_tile_nft::state::{Color, Position};

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
    TOTAL_FEES.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", config.admin))
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
        ExecuteMsg::UpdateConfig { nft_contract, admin, color_change_fee, rate_limit, rate_limit_window, requires_payment, rate_limiting_enabled } => {
            execute_update_config(deps, env, info, nft_contract, admin, color_change_fee, rate_limit, rate_limit_window, requires_payment, rate_limiting_enabled)
        }
        ExecuteMsg::WithdrawFees { amount } => {
            execute_withdraw_fees(deps, env, info, amount)
        }
        ExecuteMsg::HandleNFTTransfer { token_id, from, to } => {
            execute_handle_nft_transfer(deps, env, info, token_id, from, to)
        }
        ExecuteMsg::BatchGrantPermission { permissions } => {
            execute_batch_grant_permission(deps, env, info, permissions)
        }
        ExecuteMsg::BatchRevokePermission { permissions } => {
            execute_batch_revoke_permission(deps, env, info, permissions)
        }
        ExecuteMsg::BatchSetPublicEditing { settings } => {
            execute_batch_set_public_editing(deps, env, info, settings)
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
    
    // Query NFT contract for current owner
    let token_id = format!("tile_{}_{}",  position.x, position.y);
    let owner: String = deps.querier.query_wasm_smart(
        config.nft_contract.clone(),
        &mosaic_tile_nft::msg::QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        },
    )?;

    // Load or create tile permissions with verified owner
    let permissions = TILE_PERMISSIONS
        .may_load(deps.storage, (position.x, position.y))?
        .unwrap_or_else(|| TilePermissions {
            owner: deps.api.addr_validate(&owner).unwrap(),
            allowed_editors: vec![],
            public_editing: false,
            permission_expiry: None,
            public_change_fee: None,
        });

    // Check permissions
    if !can_change_color(&permissions, &info.sender, &env.block.time) {
        return Err(ContractError::Unauthorized {});
    }

    // Check and update rate limit
    let mut stats = USER_STATS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    if !check_rate_limit(&stats, &config, env.block.time) {
        return Err(ContractError::RateLimitExceeded {
            seconds: config.rate_limit_window,
        });
    }

    // Handle payment
    let required_fee = if config.requires_payment {
        if let Some(public_fee) = permissions.public_change_fee {
            if !permissions.owner.eq(&info.sender) {
                public_fee
            } else {
                config.color_change_fee
            }
        } else {
            config.color_change_fee
        }
    } else {
        Uint128::zero()
    };

    if required_fee > Uint128::zero() {
        let payment = cw_utils::must_pay(&info, "ustars")?;
        if payment < required_fee {
            return Err(ContractError::InsufficientPayment {
                required: required_fee.u128(),
                sent: payment.u128(),
            });
        }
    }

    // Update user statistics
    stats.total_color_changes += 1;
    stats.total_fees_paid += required_fee;
    stats.last_color_change = Some(env.block.time);
    stats.changes_in_window += 1;
    if stats.current_window_start.is_none() {
        stats.current_window_start = Some(env.block.time);
    }
    USER_STATS.save(deps.storage, &info.sender, &stats)?;

    // Update total fees
    if required_fee > Uint128::zero() {
        TOTAL_FEES.update(deps.storage, |fees| -> StdResult<_> {
            Ok(fees + required_fee)
        })?;
    }

    // Create color update event
    let color_event = Event::new("tile_color_update")
        .add_attribute("token_id", position.x.to_string() + "," + &position.y.to_string())
        .add_attribute("color_r", color.r.to_string())
        .add_attribute("color_g", color.g.to_string())
        .add_attribute("color_b", color.b.to_string())
        .add_attribute("updater", info.sender.to_string())
        .add_attribute("timestamp", env.block.time.seconds().to_string());

    Ok(Response::new()
        .add_event(color_event)
        .add_attribute("action", "change_color")
        .add_attribute("position_x", position.x.to_string())
        .add_attribute("position_y", position.y.to_string()))
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
        |existing| -> Result<_, ContractError> {
            let mut permissions = existing.unwrap_or_else(|| TilePermissions {
                owner: info.sender.clone(),
                allowed_editors: vec![],
                public_editing: false,
                permission_expiry: None,
                public_change_fee: None,
            });

            // Only owner can grant permissions
            if permissions.owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            // Check if editor is already in the list
            if permissions.allowed_editors.contains(&editor_addr) {
                return Err(ContractError::PermissionAlreadyGranted { 
                    address: editor.clone() 
                });
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
        |existing| -> Result<_, ContractError> {
            let mut permissions = existing.ok_or(ContractError::PermissionNotFound {
                address: editor.clone(),
            })?;

            // Only owner can revoke permissions
            if permissions.owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            // Remove editor from allowed list
            permissions.allowed_editors.retain(|addr| addr != &editor_addr);

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
        |existing| -> Result<_, ContractError> {
            let mut permissions = existing.unwrap_or_else(|| TilePermissions {
                owner: info.sender.clone(),
                allowed_editors: vec![],
                public_editing: false,
                permission_expiry: None,
                public_change_fee: None,
            });

            // Only owner can change public editing settings
            if permissions.owner != info.sender {
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
    _env: Env,
    info: MessageInfo,
    nft_contract: Option<String>,
    admin: Option<String>,
    color_change_fee: Option<Uint128>,
    rate_limit: Option<u32>,
    rate_limit_window: Option<u64>,
    requires_payment: Option<bool>,
    rate_limiting_enabled: Option<bool>,
) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
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
        if let Some(payment_required) = requires_payment {
            config.requires_payment = payment_required;
        }
        if let Some(rate_limiting) = rate_limiting_enabled {
            config.rate_limiting_enabled = rate_limiting;
        }

        Ok(config)
    })?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_withdraw_fees(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let withdraw_amount = TOTAL_FEES.update(deps.storage, |fees| -> Result<_, ContractError> {
        let amount_to_withdraw = amount.unwrap_or(fees);
        if amount_to_withdraw > fees {
            return Err(ContractError::InsufficientFunds {});
        }
        Ok(fees - amount_to_withdraw)
    })?;

    let bank_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![coin(withdraw_amount.u128(), "ustars")],
    });

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw_fees")
        .add_attribute("amount", withdraw_amount.to_string()))
}

pub fn execute_handle_nft_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    from: String,
    to: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only NFT contract can call this
    if info.sender != config.nft_contract {
        return Err(ContractError::Unauthorized {});
    }

    // Parse token_id to get position (format: "tile_x_y")
    let parts: Vec<&str> = token_id.split('_').collect();
    if parts.len() != 3 || parts[0] != "tile" {
        return Err(ContractError::InvalidTokenId {});
    }

    let x: u32 = parts[1].parse().map_err(|_| ContractError::InvalidTokenId {})?;
    let y: u32 = parts[2].parse().map_err(|_| ContractError::InvalidTokenId {})?;
    let position = Position { x, y };

    // Update tile permissions
    TILE_PERMISSIONS.update(
        deps.storage,
        (position.x, position.y),
        |existing| -> Result<_, ContractError> {
            let mut permissions = existing.unwrap_or_else(|| TilePermissions {
                owner: deps.api.addr_validate(&to)?,
                allowed_editors: vec![],
                public_editing: false,
                permission_expiry: None,
                public_change_fee: None,
            });

            // Update owner
            permissions.owner = deps.api.addr_validate(&to)?;
            // Clear allowed editors on transfer
            permissions.allowed_editors.clear();
            // Reset public editing on transfer
            permissions.public_editing = false;
            permissions.public_change_fee = None;

            Ok(permissions)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "handle_nft_transfer")
        .add_attribute("token_id", token_id)
        .add_attribute("from", from)
        .add_attribute("to", to))
}

pub fn execute_batch_grant_permission(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    permissions: Vec<(Position, String, Option<Timestamp>)>,
) -> Result<Response, ContractError> {
    let mut attrs = vec![("action".to_string(), "batch_grant_permission".to_string())];

    for (position, editor, expires_at) in permissions {
        let editor_addr = deps.api.addr_validate(&editor)?;
        
        // Verify NFT ownership
        let token_id = format!("tile_{}_{}",  position.x, position.y);
        let owner: String = deps.querier.query_wasm_smart(
            CONFIG.load(deps.storage)?.nft_contract,
            &mosaic_tile_nft::msg::QueryMsg::OwnerOf {
                token_id: token_id.clone(),
                include_expired: None,
            },
        )?;

        if owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        TILE_PERMISSIONS.update(
            deps.storage,
            (position.x, position.y),
            |existing| -> Result<_, ContractError> {
                let mut perms = existing.unwrap_or_else(|| TilePermissions {
                    owner: info.sender.clone(),
                    allowed_editors: vec![],
                    public_editing: false,
                    permission_expiry: None,
                    public_change_fee: None,
                });

                if !perms.allowed_editors.contains(&editor_addr) {
                    perms.allowed_editors.push(editor_addr.clone());
                }
                perms.permission_expiry = expires_at;

                Ok(perms)
            },
        )?;

        attrs.push(("position".to_string(), format!("{},{}", position.x, position.y)));
        attrs.push(("editor".to_string(), editor));
    }

    Ok(Response::new().add_attributes(attrs))
}

pub fn execute_batch_revoke_permission(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    permissions: Vec<(Position, String)>,
) -> Result<Response, ContractError> {
    let mut attrs = vec![("action".to_string(), "batch_revoke_permission".to_string())];

    for (position, editor) in permissions {
        let editor_addr = deps.api.addr_validate(&editor)?;
        
        // Verify NFT ownership
        let token_id = format!("tile_{}_{}",  position.x, position.y);
        let owner: String = deps.querier.query_wasm_smart(
            CONFIG.load(deps.storage)?.nft_contract,
            &mosaic_tile_nft::msg::QueryMsg::OwnerOf {
                token_id: token_id.clone(),
                include_expired: None,
            },
        )?;

        if owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        TILE_PERMISSIONS.update(
            deps.storage,
            (position.x, position.y),
            |existing| -> Result<_, ContractError> {
                let mut perms = existing.ok_or(ContractError::PermissionNotFound {
                    address: editor.clone(),
                })?;

                perms.allowed_editors.retain(|addr| addr != &editor_addr);

                Ok(perms)
            },
        )?;

        attrs.push(("position".to_string(), format!("{},{}", position.x, position.y)));
        attrs.push(("editor".to_string(), editor));
    }

    Ok(Response::new().add_attributes(attrs))
}

pub fn execute_batch_set_public_editing(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    settings: Vec<(Position, bool, Option<Uint128>)>,
) -> Result<Response, ContractError> {
    let mut attrs = vec![("action".to_string(), "batch_set_public_editing".to_string())];

    for (position, public_editing, public_change_fee) in settings {
        // Verify NFT ownership
        let token_id = format!("tile_{}_{}",  position.x, position.y);
        let owner: String = deps.querier.query_wasm_smart(
            CONFIG.load(deps.storage)?.nft_contract,
            &mosaic_tile_nft::msg::QueryMsg::OwnerOf {
                token_id: token_id.clone(),
                include_expired: None,
            },
        )?;

        if owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        TILE_PERMISSIONS.update(
            deps.storage,
            (position.x, position.y),
            |existing| -> Result<_, ContractError> {
                let mut perms = existing.unwrap_or_else(|| TilePermissions {
                    owner: info.sender.clone(),
                    allowed_editors: vec![],
                    public_editing: false,
                    permission_expiry: None,
                    public_change_fee: None,
                });

                perms.public_editing = public_editing;
                perms.public_change_fee = public_change_fee;

                Ok(perms)
            },
        )?;

        attrs.push(("position".to_string(), format!("{},{}", position.x, position.y)));
        attrs.push(("public_editing".to_string(), public_editing.to_string()));
        if let Some(fee) = public_change_fee {
            attrs.push(("public_change_fee".to_string(), fee.to_string()));
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::TilePermissions { position } => to_binary(&query_tile_permissions(deps, position)?),
        QueryMsg::UserStatistics { address } => to_binary(&query_user_statistics(deps, address)?),
        QueryMsg::TotalFees {} => to_binary(&query_total_fees(deps)?),
        QueryMsg::CanChangeColor { position, editor } => {
            to_binary(&query_can_change_color(deps, env, position, editor)?)
        }
    }
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
        let window_end = user_stats.current_window_start
            .unwrap_or(env.block.time)
            .plus_seconds(config.rate_limit_window);
        let remaining = window_end.seconds() - env.block.time.seconds();
        
        return Ok(CanChangeColorResponse {
            can_change: false,
            reason: Some(format!("Rate limit exceeded. Try again in {} seconds", remaining)),
            required_fee: None,
        });
    }

    // Calculate required fee
    let required_fee = if config.requires_payment {
        if permissions.owner == editor_addr {
            Some(config.color_change_fee)
        } else {
            Some(permissions.public_change_fee.unwrap_or(config.color_change_fee))
        }
    } else {
        None
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
    use cosmwasm_std::{coins, Addr, Timestamp};

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

        // Owner can change color with payment
        let msg = ExecuteMsg::ChangeColor {
            position: position.clone(),
            color: Color { r: 255, g: 0, b: 0 },
        };
        let info = mock_info(USER1, &coins(1000000, "ustars"));
        let env = mock_env();
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap();
        
        // Verify color update event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_r" && attr.value == "255"));

        // Non-owner cannot change color
        let info = mock_info(USER2, &coins(1000000, "ustars"));
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_permissions() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 1, y: 1 };

        // Set up initial permissions
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

        // Editor can change color with payment
        let msg = ExecuteMsg::ChangeColor {
            position: position.clone(),
            color: Color { r: 0, g: 255, b: 0 },
        };
        let info = mock_info(USER2, &coins(1000000, "ustars"));
        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        
        // Verify color update event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == "255"));

        // Revoke permission
        let msg = ExecuteMsg::RevokePermission {
            position: position.clone(),
            editor: USER2.to_string(),
        };
        let info = mock_info(USER1, &[]);
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

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

        // Set up initial permissions
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
        
        // Verify color update event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == "255"));
    }

    #[test]
    fn test_rate_limiting() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 1, y: 1 };
        let mut env = mock_env();

        // Set up initial permissions
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

        // Make multiple color changes
        for i in 0..10 {
            let msg = ExecuteMsg::ChangeColor {
                position: position.clone(),
                color: Color { r: i as u8, g: 0, b: 0 },
            };
            let info = mock_info(USER1, &coins(1000000, "ustars"));
            execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        }

        // Next change should fail due to rate limit
        let msg = ExecuteMsg::ChangeColor {
            position,
            color: Color { r: 255, g: 0, b: 0 },
        };
        let info = mock_info(USER1, &coins(1000000, "ustars"));
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert!(matches!(err, ContractError::RateLimitExceeded { seconds: 3600 }));
    }

    #[test]
    fn test_fee_collection() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let position = Position { x: 1, y: 1 };
        let env = mock_env();

        // Set up initial permissions
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

        // Change color with fee
        let msg = ExecuteMsg::ChangeColor {
            position,
            color: Color { r: 255, g: 0, b: 0 },
        };
        let info = mock_info(USER1, &coins(1000000, "ustars"));
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Check total fees
        let res = query_total_fees(deps.as_ref()).unwrap();
        assert_eq!(res.total_fees, Uint128::from(1000000u128));

        // Withdraw fees
        let msg = ExecuteMsg::WithdrawFees { amount: None };
        let info = mock_info(ADMIN, &[]);
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        
        // Verify bank message
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Bank(BankMsg::Send { amount, .. }) => {
                assert_eq!(amount[0].amount, Uint128::from(1000000u128));
            }
            _ => panic!("Expected bank message"),
        }
    }
}