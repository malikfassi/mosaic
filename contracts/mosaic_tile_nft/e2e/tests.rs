use e2e_framework::contract::{Contract, CW721Contract};
use e2e_framework::Error;
use serde_json::json;

const PIXELS_PER_TILE: u32 = 100; // 10x10 pixels per tile
const TOTAL_TILES: u32 = 10000; // Total number of tiles

async fn get_next_free_tile_id(contract: &Contract) -> Option<u32> {
    // Query mosaic state to get total tiles minted
    let state = contract.query(&json!({
        "mosaic_state": {}
    })).await.ok()?;
    println!("State: {:?}", state);
        
    // Start checking from 0 to find first available ID
    for tile_id in 0..TOTAL_TILES {
        // Try to query owner - if it fails, the tile is available
        if contract.query_owner_of(&tile_id.to_string()).await.is_err() {
            return Some(tile_id);
        }
    }
    
    None // No free tiles found
}

async fn get_tile_pixels(contract: &Contract, tile_id: u32) -> Option<Vec<(u32, bool)>> {
    // Query all pixels in the tile to find which ones are available
    let response = contract.query(&json!({
        "tile_pixels": {
            "tile_id": tile_id
        }
    })).await.ok()?;
    println!("Tile pixels response: {:?}", response);

    let pixels = response["pixels"].as_array()?;
    let mut pixel_states = Vec::with_capacity(PIXELS_PER_TILE as usize);

    for (i, pixel) in pixels.iter().enumerate() {
        let pixel_id = tile_id * PIXELS_PER_TILE + i as u32;
        let is_set = pixel["color"]["r"].as_u64().unwrap_or(0) != 0 
            || pixel["color"]["g"].as_u64().unwrap_or(0) != 0 
            || pixel["color"]["b"].as_u64().unwrap_or(0) != 0;
        pixel_states.push((pixel_id, is_set));
    }

    Some(pixel_states)
}

async fn get_free_pixel_in_tile(contract: &Contract, tile_id: u32) -> Option<u32> {
    let pixels = get_tile_pixels(contract, tile_id).await?;
    println!("Pixel states: {:?}", pixels);
    // Find first unset pixel
    pixels.into_iter()
        .find(|(_, is_set)| !is_set)
        .map(|(pixel_id, _)| pixel_id)
}

async fn query_pixel_state(contract: &Contract, pixel_id: u32) -> Result<serde_json::Value, Error> {
    println!("Querying pixel state for ID: {}", pixel_id);
    let query = json!({
        "pixel_state": {
            "pixel_id": pixel_id
        }
    });
    println!("Query: {}", query);
    let result = contract.query(&query).await;
    println!("Query result: {:?}", result);
    result
}

#[tokio::test]
async fn test_mint_tile() {
    // Initialize contract and addresses from environment
    let contract = Contract::from_env("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
    let minter = std::env::var("MINTER_ADDRESS").expect("MINTER_ADDRESS must be set");
    let owner = std::env::var("OWNER_ADDRESS").expect("OWNER_ADDRESS must be set");

    // Get next available tile ID from contract state
    let tile_id = get_next_free_tile_id(&contract)
        .await
        .expect("Failed to get next free tile ID");
    println!("Using tile ID: {}", tile_id);

    // Try to mint from non-minter (should fail)
    let msg = json!({
        "cw721": {
            "mint": {
                "token_id": tile_id.to_string(),
                "owner": owner,
                "token_uri": null,
                "extension": {}
            }
        }
    });

    let result = contract.execute(&msg, &owner, None).await;
    assert!(result.is_err(), "Non-minter should not be able to mint");

    // Mint a tile using minter account
    let result = contract.execute(&msg, &minter, None).await;
    assert!(result.is_ok(), "Minter should be able to mint");

    // Query the owner
    let queried_owner = contract.query_owner_of(&tile_id.to_string())
        .await
        .expect("Failed to query owner");
    
    assert_eq!(queried_owner, owner, "Owner should match");

    // Try to mint the same tile again - should fail
    let result = contract.execute(&msg, &minter, None).await;
    assert!(result.is_err(), "Should not be able to mint the same tile twice");
}

#[tokio::test]
async fn test_set_pixel_color() {
    // Initialize contract and addresses from environment
    let contract = Contract::from_env("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
    let minter = std::env::var("MINTER_ADDRESS").expect("MINTER_ADDRESS must be set");
    let owner = std::env::var("OWNER_ADDRESS").expect("OWNER_ADDRESS must be set");
    let user = std::env::var("USER_ADDRESS").expect("USER_ADDRESS must be set");

    // First mint a tile
    let tile_id = get_next_free_tile_id(&contract)
        .await
        .expect("Failed to get next free tile ID");
    println!("Using tile ID: {}", tile_id);
    
    let msg = json!({
        "cw721": {
            "mint": {
                "token_id": tile_id.to_string(),
                "owner": owner,
                "token_uri": null,
                "extension": {}
            }
        }
    });

    contract.execute(&msg, &minter, None)
        .await
        .expect("Failed to mint tile");

    // Find a free pixel in the tile
    let pixel_id = get_free_pixel_in_tile(&contract, tile_id)
        .await
        .expect("No free pixels in tile");
    println!("Setting color for pixel ID: {}", pixel_id);
    
    let msg = json!({
        "set_pixel_color": {
            "pixel_id": pixel_id,
            "color": {
                "r": 255,
                "g": 0,
                "b": 0
            }
        }
    });

    // Get required fees from contract state
    let state = contract.query(&json!({
        "mosaic_state": {}
    })).await.expect("Failed to query mosaic state");
    println!("Contract state: {:?}", state);

    let developer_fee = state["developer_fee"]["amount"].as_str().unwrap();
    let owner_fee = state["owner_fee"]["amount"].as_str().unwrap();
    let total_fee = (developer_fee.parse::<u64>().unwrap() + owner_fee.parse::<u64>().unwrap()).to_string() + "ustars";

    println!("Sending fee: {}", total_fee);

    // Try to set color without fees (should fail)
    let result = contract.execute(&msg, &user, None).await;
    assert!(result.is_err(), "Should not be able to set color without fees");

    // Set color with proper fees
    let execute_result = contract.execute(&msg, &user, Some(&total_fee)).await;
    println!("Execute result: {:?}", execute_result);
    execute_result.expect("Failed to set pixel color");

    // Query the pixel state
    let pixel_state = query_pixel_state(&contract, pixel_id)
        .await
        .expect("Failed to query pixel state");
    println!("Pixel state after setting color: {:?}", pixel_state);

    assert_eq!(pixel_state["tile_id"].as_u64().unwrap(), tile_id as u64);
    assert_eq!(pixel_state["owner"].as_str().unwrap(), owner);
    assert_eq!(pixel_state["color"]["r"].as_u64().unwrap(), 255);
    assert_eq!(pixel_state["color"]["g"].as_u64().unwrap(), 0);
    assert_eq!(pixel_state["color"]["b"].as_u64().unwrap(), 0);

    // Try to set color for a pixel in a non-existent tile - should fail
    let invalid_pixel_id = (tile_id + 1) * PIXELS_PER_TILE; // First pixel in next (non-existent) tile
    println!("Trying to set color for invalid pixel ID: {}", invalid_pixel_id);
    
    let msg = json!({
        "set_pixel_color": {
            "pixel_id": invalid_pixel_id,
            "color": {
                "r": 255,
                "g": 0,
                "b": 0
            }
        }
    });

    let result = contract.execute(&msg, &user, Some(&total_fee)).await;
    println!("Invalid pixel result: {:?}", result);
    assert!(result.is_err(), "Should not be able to set color for a pixel in a non-existent tile");
}

#[tokio::test]
async fn test_fee_distribution() {
    // Initialize contract and addresses from environment
    let contract = Contract::from_env("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
    let minter = std::env::var("MINTER_ADDRESS").expect("MINTER_ADDRESS must be set");
    let owner = std::env::var("OWNER_ADDRESS").expect("OWNER_ADDRESS must be set");
    let user = std::env::var("USER_ADDRESS").expect("USER_ADDRESS must be set");
    let deployer = std::env::var("DEPLOYER_ADDRESS").expect("DEPLOYER_ADDRESS must be set");

    // First mint a tile
    let tile_id = get_next_free_tile_id(&contract)
        .await
        .expect("Failed to get next free tile ID");
    
    let msg = json!({
        "cw721": {
            "mint": {
                "token_id": tile_id.to_string(),
                "owner": owner,
                "token_uri": null,
                "extension": {}
            }
        }
    });

    contract.execute(&msg, &minter, None)
        .await
        .expect("Failed to mint tile");

    // Get a free pixel
    let pixel_id = get_free_pixel_in_tile(&contract, tile_id)
        .await
        .expect("No free pixels in tile");

    // Query initial balances
    let initial_deployer_balance = contract.query_balance(&deployer, "ustars").await.expect("Failed to query deployer balance");
    let initial_owner_balance = contract.query_balance(&owner, "ustars").await.expect("Failed to query owner balance");

    // Set pixel color
    let msg = json!({
        "set_pixel_color": {
            "pixel_id": pixel_id,
            "color": {
                "r": 255,
                "g": 0,
                "b": 0
            }
        }
    });

    // Get required fees
    let state = contract.query(&json!({
        "mosaic_state": {}
    })).await.expect("Failed to query mosaic state");

    let developer_fee = state["developer_fee"]["amount"].as_str().unwrap().parse::<u64>().unwrap();
    let owner_fee = state["owner_fee"]["amount"].as_str().unwrap().parse::<u64>().unwrap();
    let total_fee = (developer_fee + owner_fee).to_string() + "ustars";

    // Execute color change
    contract.execute(&msg, &user, Some(&total_fee))
        .await
        .expect("Failed to set pixel color");

    // Query final balances
    let final_deployer_balance = contract.query_balance(&deployer, "ustars").await.expect("Failed to query deployer balance");
    let final_owner_balance = contract.query_balance(&owner, "ustars").await.expect("Failed to query owner balance");

    // Calculate differences
    let deployer_diff = final_deployer_balance.parse::<u64>().unwrap() - initial_deployer_balance.parse::<u64>().unwrap();
    let owner_diff = final_owner_balance.parse::<u64>().unwrap() - initial_owner_balance.parse::<u64>().unwrap();

    // Get fee amounts from state
    let state = contract.query(&json!({
        "mosaic_state": {}
    })).await.expect("Failed to query mosaic state");
    let developer_fee = state["developer_fee"]["amount"].as_str().unwrap().parse::<u64>().unwrap();
    let owner_fee = state["owner_fee"]["amount"].as_str().unwrap().parse::<u64>().unwrap();

    // Assert the differences match the fees
    assert_eq!(deployer_diff, developer_fee);
    assert_eq!(owner_diff, owner_fee);
}
