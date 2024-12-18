use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    testing::{mock_dependencies, mock_env, mock_info},
    Binary, OwnedDeps, SystemResult, ContractResult, coins, BankMsg, to_json_binary, Empty, from_json, Uint128, Coin,
};

use crate::{
    execute, instantiate,
    msg::{ExecuteMsg, PixelUpdate},
    error::ContractError,
    constants::fees,
    types::TileMetadata,
};
use sg721::InstantiateMsg;

const MINTER: &str = "minter";
const PIXELS_PER_TILE: u32 = 100;
const TILE_OWNER: &str = "tile_owner";

fn mock_dependencies_with_querier() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();
    deps.querier.update_wasm(|query| {
        match query {
            cosmwasm_std::WasmQuery::Smart { contract_addr: _, msg } => {
                // Parse the message
                let query_msg: cw721::Cw721QueryMsg = from_json(msg).unwrap();
                match query_msg {
                    cw721::Cw721QueryMsg::OwnerOf { token_id: _, include_expired: _ } => {
                        // Return mock owner response
                        let response = cw721::OwnerOfResponse {
                            owner: TILE_OWNER.to_string(),
                            approvals: vec![],
                        };
                        SystemResult::Ok(ContractResult::Ok(to_json_binary(&response).unwrap()))
                    }
                    _ => SystemResult::Ok(ContractResult::Ok(Binary::from(br#"{"code_id":1,"creator":"creator","admin":null,"pinned":false,"ibc_port":null}"#.to_vec()))),
                }
            }
            _ => SystemResult::Ok(ContractResult::Ok(Binary::from(br#"{"code_id":1,"creator":"creator","admin":null,"pinned":false,"ibc_port":null}"#.to_vec()))),
        }
    });
    deps
}

fn setup_contract(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Initialize the contract
    let msg = InstantiateMsg {
        name: "MosaicTiles".to_string(),
        symbol: "TILE".to_string(),
        minter: MINTER.to_string(),
        collection_info: sg721::CollectionInfo {
            creator: MINTER.to_string(),
            description: "A mosaic NFT collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            royalty_info: None,
            explicit_content: Some(false),
            start_trading_time: None,
        },
    };

    instantiate(deps.as_mut(), env, info, msg).unwrap();
}

#[test]
fn set_pixel_color_invalid_pixel() {
    let mut deps = mock_dependencies_with_querier();
    let env = mock_env();

    // Initialize the contract
    let msg = InstantiateMsg {
        name: "MosaicTiles".to_string(),
        symbol: "TILE".to_string(),
        minter: MINTER.to_string(),
        collection_info: sg721::CollectionInfo {
            creator: MINTER.to_string(),
            description: "A mosaic NFT collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            royalty_info: None,
            explicit_content: Some(false),
            start_trading_time: None,
        },
    };

    let info = mock_info(MINTER, &[]); // No funds for instantiate
    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Create a new tile
    let tile = TileMetadata::new(0, env.block.time.seconds());
    let current_tile_metadata = tile.to_bytes();

    // Calculate the required fee for 1 hour duration
    let duration = 3600u64; // 1 hour
    let required_fee = fees::calculate_fee(duration);

    // Try to update a pixel outside the tile range
    let msg = ExecuteMsg::SetPixelColor {
        current_tile_metadata,
        pixel_update: PixelUpdate {
            pixel_id: PIXELS_PER_TILE + 1, // Invalid pixel ID
            color: [255, 0, 0],
            expiration: env.block.time.seconds() + duration,
        },
    };

    // Execute the message with the required fee
    let info = mock_info(MINTER, &[required_fee]);
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    println!("Got error: {:?}", err); // Add debug output
    assert!(matches!(err, ContractError::PixelOutOfRange {}));
}

#[test]
fn burn_is_disabled() {
    let mut deps = mock_dependencies_with_querier();
    setup_contract(&mut deps);
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Try to burn a token
    let msg = ExecuteMsg::Base(sg721::ExecuteMsg::Burn {
        token_id: "1".to_string(),
    });

    // Execute the message
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert!(matches!(err, ContractError::FeatureDisabled { feature } if feature == "burn"));
}

#[test]
fn set_pixel_color_fee_tiers() {
    let mut deps = mock_dependencies_with_querier();
    let env = mock_env();

    // Initialize the contract
    let msg = InstantiateMsg {
        name: "MosaicTiles".to_string(),
        symbol: "TILE".to_string(),
        minter: MINTER.to_string(),
        collection_info: sg721::CollectionInfo {
            creator: MINTER.to_string(),
            description: "A mosaic NFT collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            royalty_info: None,
            explicit_content: Some(false),
            start_trading_time: None,
        },
    };

    let info = mock_info(MINTER, &[]); // No funds for instantiate
    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Create a new tile
    let tile = TileMetadata::new(0, env.block.time.seconds());
    let current_tile_metadata = tile.to_bytes();

    // Test different fee tiers
    let test_cases = vec![
        // (duration, expected_fee)
        (30 * 60, fees::FEE_TIER_1H),           // 30 minutes -> 5 STARS
        (2 * fees::HOUR, fees::FEE_TIER_12H),   // 2 hours -> 10 STARS
        (13 * fees::HOUR, fees::FEE_TIER_24H),  // 13 hours -> 15 STARS
        (2 * fees::DAY, 4 * fees::FEE_TIER_24H), // 2 days -> 60 STARS (quadratic scaling)
    ];

    for (duration, expected_fee) in test_cases {
        let msg = ExecuteMsg::SetPixelColor {
            current_tile_metadata: current_tile_metadata.clone(),
            pixel_update: PixelUpdate {
                pixel_id: 1,
                color: [255, 0, 0],
                expiration: env.block.time.seconds() + duration,
            },
        };

        // Try with incorrect fee
        let wrong_fee = expected_fee - 1_000_000;
        let info = mock_info(MINTER, &coins(wrong_fee, "ustars"));
        let err = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap_err();
        match err {
            ContractError::InvalidFee { expected, received } => {
                assert_eq!(expected.amount, Uint128::from(expected_fee));
                assert_eq!(received.unwrap().amount, Uint128::from(wrong_fee));
            }
            _ => panic!("Expected InvalidFee error"),
        }

        // Try with correct fee
        let info = mock_info(MINTER, &coins(expected_fee, "ustars"));
        let response = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Verify fee distribution
        let developer_royalties = fees::developer_royalties() as u128;
        let developer_amount = Uint128::from(expected_fee).multiply_ratio(developer_royalties, 100u128);
        let owner_amount = Uint128::from(expected_fee) - developer_amount;

        // Verify the bank messages
        let messages = response.messages;
        assert_eq!(messages.len(), 2);

        // Verify developer fee
        match &messages[0].msg {
            cosmwasm_std::CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                assert_eq!(to_address, &fees::developer_address());
                assert_eq!(amount.len(), 1);
                assert_eq!(amount[0].denom, "ustars");
                assert_eq!(amount[0].amount, developer_amount);
            }
            _ => panic!("Expected bank message"),
        }

        // Verify owner fee
        match &messages[1].msg {
            cosmwasm_std::CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                assert_eq!(to_address, TILE_OWNER);
                assert_eq!(amount.len(), 1);
                assert_eq!(amount[0].denom, "ustars");
                assert_eq!(amount[0].amount, owner_amount);
            }
            _ => panic!("Expected bank message"),
        }

        // Verify attributes
        let attrs = response.attributes;
        assert!(attrs.iter().any(|attr| attr.key == "fee" && attr.value == expected_fee.to_string()));
        assert!(attrs.iter().any(|attr| attr.key == "duration" && attr.value == duration.to_string()));
    }
}

#[test]
fn set_pixel_color_fee_per_second() {
    let mut deps = mock_dependencies_with_querier();
    let env = mock_env();

    // Initialize the contract
    let msg = InstantiateMsg {
        name: "MosaicTiles".to_string(),
        symbol: "TILE".to_string(),
        minter: MINTER.to_string(),
        collection_info: sg721::CollectionInfo {
            creator: MINTER.to_string(),
            description: "A mosaic NFT collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            royalty_info: None,
            explicit_content: Some(false),
            start_trading_time: None,
        },
    };

    let info = mock_info(MINTER, &[]); // No funds for instantiate
    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Create a new tile
    let tile = TileMetadata::new(0, env.block.time.seconds());
    let current_tile_metadata = tile.to_bytes();

    // Test per-second granularity for durations over 24h
    let base_duration = fees::DAY;
    let test_durations = vec![
        base_duration + 1,      // 24h + 1s
        base_duration + 3600,   // 24h + 1h
        base_duration + 7200,   // 24h + 2h
        base_duration * 2,      // 48h
    ];

    let mut last_duration: Option<u64> = None;
    let mut last_fee: Option<Coin> = None;
    for duration in test_durations {
        let msg = ExecuteMsg::SetPixelColor {
            current_tile_metadata: current_tile_metadata.clone(),
            pixel_update: PixelUpdate {
                pixel_id: 1,
                color: [255, 0, 0],
                expiration: env.block.time.seconds() + duration,
            },
        };

        // Calculate fee
        let fee = fees::calculate_fee(duration);
        
        // Verify fee increases with duration
        if let (Some(last_fee), Some(last_duration)) = (last_fee.as_ref(), last_duration) {
            assert!(fee.amount > last_fee.amount, "Fee should increase with duration");
            
            // Verify the fee difference is proportional to the duration difference
            let fee_ratio = fee.amount.u128() as f64 / last_fee.amount.u128() as f64;
            let duration_ratio = duration as f64 / last_duration as f64;
            let expected_ratio = duration_ratio * duration_ratio; // quadratic scaling
            
            // Allow for some floating point imprecision
            assert!((fee_ratio - expected_ratio).abs() < 0.0001, 
                "Fee scaling should be quadratic. Expected ratio: {}, got: {}", 
                expected_ratio, fee_ratio);
        }
        last_fee = Some(fee.clone());
        last_duration = Some(duration);

        // Try with the calculated fee
        let info = mock_info(MINTER, &[fee.clone()]);
        let response = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Verify attributes
        let attrs = response.attributes;
        assert!(attrs.iter().any(|attr| attr.key == "fee" && attr.value == fee.amount.to_string()));
        assert!(attrs.iter().any(|attr| attr.key == "duration" && attr.value == duration.to_string()));
    }
} 