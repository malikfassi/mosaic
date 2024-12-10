use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    coins, from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    Coin, OwnedDeps, Response, Uint128,
};

use crate::{
    contract::{execute, instantiate, query},
    msg::{
        ExecuteMsg, InstantiateMsg, PixelStateResponse, PixelUpdate, QueryMsg, TilePixelsResponse,
        TileStateResponse,
    },
    state::{Color, PIXELS_PER_TILE, TOKEN_COUNT},
};

// Helper functions
fn create_color(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}

const MINTER: &str = "minter";
const DEVELOPER: &str = "developer";
const OWNER: &str = "owner";

fn setup_contract() -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, Response) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    let msg = InstantiateMsg {
        name: "MosaicTiles".to_string(),
        symbol: "TILE".to_string(),
        minter: MINTER.to_string(),
        developer: DEVELOPER.to_string(),
        collection_info: sg721::CollectionInfo {
            creator: MINTER.to_string(),
            description: "Mosaic Tile NFTs".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            explicit_content: None,
            start_trading_time: None,
            royalty_info: None,
        },
        developer_fee: Coin {
            denom: "ustars".to_string(),
            amount: Uint128::from(100u128),
        },
        owner_fee: Coin {
            denom: "ustars".to_string(),
            amount: Uint128::from(100u128),
        },
    };

    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    (deps, res)
}

#[test]
fn proper_initialization() {
    let (deps, res) = setup_contract();
    assert_eq!(0, res.messages.len());

    // Check token count
    let count = TOKEN_COUNT.load(&deps.storage).unwrap();
    assert_eq!(0, count);
}

#[test]
fn test_mint_tile() {
    let (mut deps, _) = setup_contract();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Check initial token count
    let initial_count = TOKEN_COUNT.load(&deps.storage).unwrap();
    println!("Initial token count: {}", initial_count);

    // Mint a tile
    let tile_id = 0;
    let msg = ExecuteMsg::MintTile {
        tile_id,
        owner: OWNER.to_string(),
    };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("Mint response: {:?}", res);
    assert_eq!(1, res.messages.len());

    // Check token count after minting
    let count = TOKEN_COUNT.load(&deps.storage).unwrap();
    println!("Token count after minting: {}", count);
    assert_eq!(1, count);

    // Query tile state
    let msg = QueryMsg::TileState { tile_id };
    let res: TileStateResponse = from_json(query(deps.as_ref(), env, msg).unwrap()).unwrap();
    println!("Tile state response: {:?}", res);
    assert_eq!(OWNER, res.owner);
    assert_eq!(tile_id, res.tile_id);
    assert_eq!(PIXELS_PER_TILE as usize, res.pixel_colors.len());
}

#[test]
fn test_set_pixel_color() {
    let (mut deps, _) = setup_contract();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Mint a tile
    let tile_id = 0;
    let msg = ExecuteMsg::MintTile {
        tile_id,
        owner: OWNER.to_string(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Set pixel color
    let pixel_id = 0;
    let color = create_color(255, 0, 0);
    let msg = ExecuteMsg::SetPixelColor {
        pixel_id,
        color: color.clone(),
    };

    let info = mock_info(OWNER, &coins(200, "ustars"));
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(2, res.messages.len()); // Fee transfers

    // Query pixel state
    let msg = QueryMsg::PixelState { pixel_id };
    let res: PixelStateResponse = from_json(query(deps.as_ref(), env, msg).unwrap()).unwrap();
    assert_eq!(OWNER, res.owner);
    assert_eq!(tile_id, res.tile_id);
    assert_eq!(color, res.color);
}

#[test]
fn test_batch_set_pixels() {
    let (mut deps, _) = setup_contract();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Mint a tile
    let tile_id = 0;
    let msg = ExecuteMsg::MintTile {
        tile_id,
        owner: OWNER.to_string(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Batch set pixels
    let updates = vec![
        PixelUpdate {
            pixel_id: 0,
            color: create_color(255, 0, 0),
        },
        PixelUpdate {
            pixel_id: 1,
            color: create_color(0, 255, 0),
        },
    ];
    let msg = ExecuteMsg::BatchSetPixels { updates };

    let info = mock_info(OWNER, &coins(400, "ustars"));
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(2, res.messages.len()); // Fee transfers

    // Query tile pixels
    let msg = QueryMsg::TilePixels { tile_id };
    let res: TilePixelsResponse = from_json(query(deps.as_ref(), env, msg).unwrap()).unwrap();
    assert_eq!(OWNER, res.owner);
    assert_eq!(tile_id, res.tile_id);
    assert_eq!(PIXELS_PER_TILE as usize, res.pixels.len());
    assert_eq!(create_color(255, 0, 0), res.pixels[0].color);
    assert_eq!(create_color(0, 255, 0), res.pixels[1].color);
}

#[test]
fn test_query_pagination() {
    let (mut deps, _) = setup_contract();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Mint multiple tiles
    for i in 0..10 {
        let msg = ExecuteMsg::MintTile {
            tile_id: i,
            owner: OWNER.to_string(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    }

    // Query with pagination
    let msg = QueryMsg::PixelsState {
        pixel_ids: (0..10).collect(),
        start_after: None,
        limit: Some(5),
    };
    let res: Vec<PixelStateResponse> =
        from_json(query(deps.as_ref(), env.clone(), msg).unwrap()).unwrap();
    assert_eq!(5, res.len());

    let msg = QueryMsg::PixelsState {
        pixel_ids: (0..10).collect(),
        start_after: Some(4),
        limit: Some(5),
    };
    let res: Vec<PixelStateResponse> = from_json(query(deps.as_ref(), env, msg).unwrap()).unwrap();
    assert_eq!(5, res.len());
}

#[test]
fn test_batch_tile_queries() {
    let (mut deps, _) = setup_contract();
    let env = mock_env();
    let info = mock_info(MINTER, &[]);

    // Mint multiple tiles
    for i in 0..3 {
        let msg = ExecuteMsg::MintTile {
            tile_id: i,
            owner: OWNER.to_string(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    }

    // Query multiple tiles
    let msg = QueryMsg::BatchTilePixels {
        tile_ids: vec![0, 1, 2],
    };
    let res: Vec<TilePixelsResponse> = from_json(query(deps.as_ref(), env, msg).unwrap()).unwrap();
    assert_eq!(3, res.len());
    assert!(res
        .iter()
        .all(|t| t.pixels.len() == PIXELS_PER_TILE as usize));
}
