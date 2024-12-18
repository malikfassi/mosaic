use cosmwasm_std::{Env, MessageInfo, OwnedDeps};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
use sg721::InstantiateMsg;

/// Helper function to setup a contract for testing
pub fn setup_contract() -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, Env, MessageInfo) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &[]);

    // Create a basic sg721 instantiate message
    let msg = InstantiateMsg {
        name: "Mosaic NFT".to_string(),
        symbol: "MOSAIC".to_string(),
        minter: "creator".to_string(),
        collection_info: sg721::CollectionInfo {
            creator: "creator".to_string(),
            description: "A mosaic NFT collection".to_string(),
            image: "https://example.com/image.png".to_string(),
            external_link: None,
            royalty_info: None,
            explicit_content: Some(false),
            start_trading_time: None,
        },
    };

    // Initialize contract
    mosaic_contract::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    (deps, env, info)
}
  