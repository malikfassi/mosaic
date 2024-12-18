use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    coins, from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    Coin, OwnedDeps, Response, Uint128,
};
use sg721::InstantiateMsg;
use sg721::CollectionInfo;
use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, PixelUpdate},
    types::{TileMetadata, pixels_per_tile},
    error::ContractError,
    constants::fees,
};

const MINTER: &str = "minter";

fn setup_contract() -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, Response) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    let msg = InstantiateMsg {
        name: "MosaicTiles".to_string(),
        symbol: "TILE".to_string(),
        minter: MINTER.to_string(),
        collection_info: CollectionInfo {
            creator: MINTER.to_string(),
            description: "A mosaic NFT collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            royalty_info: None,
            explicit_content: Some(false),
            start_trading_time: None,
        },
    };

    let response = instantiate(deps.as_mut(), env, info, msg)
        .expect("Contract initialization failed");

    (deps, response)
}

#[test]
fn set_pixel_color_invalid_pixel() {
    let (mut deps, _response) = setup_contract();
    let env = mock_env();

    // Create a new tile
    let tile = TileMetadata::new(0, env.block.time.seconds());
    let current_tile_metadata = tile.to_bytes();

    // Try to update a pixel outside the tile range
    let pixel_update = PixelUpdate {
        pixel_id: pixels_per_tile() + 1, // Invalid pixel ID
        color: [255, 0, 0],
        expiration: env.block.time.seconds() + 3600,
    };

    let msg = ExecuteMsg::SetPixelColor {
        current_tile_metadata,
        pixel_update,
    };

    // Execute the message
    let info = mock_info(MINTER, &[fees::base_fee()]);
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert!(matches!(err, ContractError::PixelOutOfRange {}));
} 