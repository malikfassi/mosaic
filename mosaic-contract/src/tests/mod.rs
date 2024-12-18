use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    testing::{mock_dependencies, mock_env, mock_info},
    Binary, OwnedDeps, SystemResult, ContractResult, coins,
};

use crate::{
    execute, instantiate,
    msg::{ExecuteMsg, PixelUpdate},
    error::ContractError,
    constants::fees,
};
use sg721::InstantiateMsg;

const MINTER: &str = "minter";
const PIXELS_PER_TILE: u32 = 100;

fn mock_dependencies_with_querier() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();
    deps.querier.update_wasm(|_| {
        let response = Binary::from(br#"{"code_id":1,"creator":"creator","admin":null,"pinned":false,"ibc_port":null}"#.to_vec());
        SystemResult::Ok(ContractResult::Ok(response))
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
    let current_tile_metadata = vec![0u8; 32]; // Mock metadata

    // Try to update a pixel outside the tile range
    let msg = ExecuteMsg::SetPixelColor {
        current_tile_metadata,
        pixel_update: PixelUpdate {
            pixel_id: PIXELS_PER_TILE + 1, // Invalid pixel ID
            color: [255, 0, 0],
            expiration: env.block.time.seconds() + 3600,
        },
    };

    // Execute the message with the required fee
    let info = mock_info(MINTER, &coins(20000000, "ustars")); // Add base fee for SetPixelColor
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