use cosmwasm_std::{Addr, Coin};
use sg_std::StargazeMsgWrapper;
use sg721::CollectionInfo;

const CONTRACT_NAME: &str = "Mosaic Tile NFT";
const SYMBOL: &str = "TILE";

#[test]
async fn test_deploy_and_mint() {
    // Get contract address from deploy job output
    let contract_addr = std::env::var("CONTRACT_ADDRESS")
        .expect("CONTRACT_ADDRESS must be set");
    let deployer_addr = std::env::var("DEPLOYMENT_WALLET_ADDRESS")
        .expect("DEPLOYMENT_WALLET_ADDRESS must be set");

    // Create client
    let client = starsd::Client::new(
        "https://rpc.elgafar-1.stargaze-apis.com:443",
        "elgafar-1",
    );

    // Mint a tile
    let mint_msg = ExecuteMsg::MintTile {
        tile_id: 1,
        owner: deployer_addr.clone(),
    };

    let resp = client
        .execute(&contract_addr, &mint_msg, &[])
        .await
        .expect("Failed to mint tile");

    // Query tile state
    let tile_state: TileStateResponse = client
        .query(
            &contract_addr,
            &QueryMsg::TileState { tile_id: 1 },
        )
        .await
        .expect("Failed to query tile state");

    assert_eq!(tile_state.owner, deployer_addr);
    assert_eq!(tile_state.tile_id, 1);

    // Update pixel color
    let update_msg = ExecuteMsg::SetPixelColor {
        pixel_id: 0,
        color: Color { r: 255, g: 0, b: 0 },
    };

    client
        .execute(&contract_addr, &update_msg, &[])
        .await
        .expect("Failed to update pixel color");

    // Query pixel state
    let pixel_state: PixelStateResponse = client
        .query(
            &contract_addr,
            &QueryMsg::PixelState { pixel_id: 0 },
        )
        .await
        .expect("Failed to query pixel state");

    assert_eq!(pixel_state.color.r, 255);
    assert_eq!(pixel_state.color.g, 0);
    assert_eq!(pixel_state.color.b, 0);
} 