use cosmwasm_std::{coins, Uint128};
use mosaic_tile_nft::state::{Position, Color};

use crate::{
    error::ContractError,
    testing::{constants::*, setup::*},
};

#[test]
fn test_initialization() {
    let ctx = setup_test_case();

    // Verify config
    let config = crate::state::CONFIG.load(&ctx.deps.storage).unwrap();
    assert_eq!(config.nft_contract, mock_nft_contract());
    assert_eq!(config.admin, mock_admin());
    assert_eq!(config.color_change_fee, default_color_change_fee());
    assert_eq!(config.rate_limit, DEFAULT_RATE_LIMIT);
    assert_eq!(config.rate_limit_window, DEFAULT_RATE_LIMIT_WINDOW);
    assert!(!config.requires_payment);
    assert!(config.rate_limiting_enabled);
}

#[test]
fn test_rate_limiting() {
    let mut ctx = setup_test_case_with_config(
        Uint128::zero(),
        3, // rate limit of 3 changes
        3600, // 1 hour window
        false,
        true,
    );

    // Setup test token
    let position = test_position(0);
    let initial_color = test_color(0);
    ctx.deps.querier.mock_nft_token(
        position.clone(),
        MOCK_USER1.to_string(),
        initial_color.clone(),
    );

    // First three changes should succeed
    for i in 1..=3 {
        let color = test_color(i);
        let res = ctx.change_color(MOCK_USER1, position.clone(), color, vec![]);
        assert!(res.is_ok());
    }

    // Fourth change should fail due to rate limit
    let color = test_color(4);
    let err = ctx.change_color(MOCK_USER1, position.clone(), color, vec![]).unwrap_err();
    assert!(matches!(err, ContractError::RateLimitExceeded { .. }));

    // Advance time past the window
    ctx.advance_time(3601);

    // Should be able to change color again
    let color = test_color(5);
    let res = ctx.change_color(MOCK_USER1, position.clone(), color, vec![]);
    assert!(res.is_ok());
}

#[test]
fn test_payment_handling() {
    let mut ctx = setup_test_case_with_config(
        Uint128::from(1000000u128),
        10,
        3600,
        true,
        false,
    );

    // Setup test token
    let position = test_position(0);
    let initial_color = test_color(0);
    ctx.deps.querier.mock_nft_token(
        position.clone(),
        MOCK_USER1.to_string(),
        initial_color.clone(),
    );

    // Change color without payment should fail
    let color = test_color(1);
    let err = ctx.change_color(MOCK_USER1, position.clone(), color.clone(), vec![]).unwrap_err();
    assert!(matches!(err, ContractError::InsufficientPayment { .. }));

    // Change color with insufficient payment should fail
    let err = ctx
        .change_color(
            MOCK_USER1,
            position.clone(),
            color.clone(),
            coins(500000, "ustars"),
        )
        .unwrap_err();
    assert!(matches!(err, ContractError::InsufficientPayment { .. }));

    // Change color with correct payment should succeed
    let res = ctx.change_color(
        MOCK_USER1,
        position.clone(),
        color,
        coins(1000000, "ustars"),
    );
    assert!(res.is_ok());
}

#[test]
fn test_public_editing() {
    let mut ctx = setup_test_case();

    // Setup test token
    let position = test_position(0);
    let initial_color = test_color(0);
    ctx.deps.querier.mock_nft_token(
        position.clone(),
        MOCK_USER1.to_string(),
        initial_color.clone(),
    );

    // USER2 cannot change color initially
    let color = test_color(1);
    let err = ctx.change_color(MOCK_USER2, position.clone(), color.clone(), vec![]).unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // Enable public editing with fee
    let public_fee = Uint128::from(500000u128);
    let res = ctx.set_public_editing(MOCK_USER1, position.clone(), true, Some(public_fee));
    assert!(res.is_ok());

    // USER2 cannot change color without payment
    let err = ctx.change_color(MOCK_USER2, position.clone(), color.clone(), vec![]).unwrap_err();
    assert!(matches!(err, ContractError::InsufficientPayment { .. }));

    // USER2 can change color with payment
    let res = ctx.change_color(
        MOCK_USER2,
        position.clone(),
        color.clone(),
        coins(500000, "ustars"),
    );
    assert!(res.is_ok());

    // Owner can still change color without payment
    let color = test_color(2);
    let res = ctx.change_color(MOCK_USER1, position.clone(), color, vec![]);
    assert!(res.is_ok());
}

#[test]
fn test_permission_expiry() {
    let mut ctx = setup_test_case();

    // Setup test token
    let position = test_position(0);
    let initial_color = test_color(0);
    ctx.deps.querier.mock_nft_token(
        position.clone(),
        MOCK_USER1.to_string(),
        initial_color.clone(),
    );

    // Grant permission with expiry
    let expires_at = ctx.env.block.time.plus_seconds(3600); // 1 hour
    let info = mock_info(MOCK_USER1, &[]);
    let msg = crate::msg::ExecuteMsg::GrantPermission {
        position: position.clone(),
        editor: MOCK_USER2.to_string(),
        expires_at: Some(expires_at),
    };
    let res = crate::contract::execute(ctx.deps.as_mut(), ctx.env.clone(), info, msg);
    assert!(res.is_ok());

    // USER2 can change color before expiry
    let color = test_color(1);
    let res = ctx.change_color(MOCK_USER2, position.clone(), color.clone(), vec![]);
    assert!(res.is_ok());

    // Advance time past expiry
    ctx.advance_time(3601);

    // USER2 cannot change color after expiry
    let color = test_color(2);
    let err = ctx.change_color(MOCK_USER2, position.clone(), color, vec![]).unwrap_err();
    assert!(matches!(err, ContractError::PermissionExpired {}));
}

#[test]
fn test_nft_ownership_verification() {
    let mut ctx = setup_test_case();

    // Setup test token
    let position = test_position(0);
    let initial_color = test_color(0);
    ctx.deps.querier.mock_nft_token(
        position.clone(),
        MOCK_USER1.to_string(),
        initial_color.clone(),
    );

    // USER1 can change color
    let color = test_color(1);
    let res = ctx.change_color(MOCK_USER1, position.clone(), color.clone(), vec![]);
    assert!(res.is_ok());

    // Change token owner to USER2
    ctx.deps.querier.mock_nft_token(
        position.clone(),
        MOCK_USER2.to_string(),
        color.clone(),
    );

    // USER1 can no longer change color
    let color = test_color(2);
    let err = ctx.change_color(MOCK_USER1, position.clone(), color.clone(), vec![]).unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // USER2 can change color
    let res = ctx.change_color(MOCK_USER2, position.clone(), color, vec![]);
    assert!(res.is_ok());
} 