use cosmwasm_std::{testing::mock_env, testing::mock_info, Env, MessageInfo, OwnedDeps};
use sg721::CollectionInfo;

use crate::{
    contract::instantiate,
    msg::InstantiateMsg,
    state::{Color, Position},
    testing::{
        constants::{
            COLLECTION_DESCRIPTION, COLLECTION_IMAGE, COLLECTION_NAME,
            COLLECTION_SYMBOL, CREATOR, MINTER,
        },
        mock_querier::{mock_deps, CustomMockQuerier},
    },
};
use cosmwasm_std::{Empty, MockApi, MockStorage};

pub fn setup_contract() -> (OwnedDeps<MockStorage, MockApi, CustomMockQuerier, Empty>, Env) {
    let mut deps = mock_deps();
    let env = mock_env();

    // Instantiate contract
    let info = mock_info(CREATOR, &[]);
    let init_msg = InstantiateMsg {
        name: COLLECTION_NAME.to_string(),
        symbol: COLLECTION_SYMBOL.to_string(),
        minter: MINTER.to_string(),
        collection_info: CollectionInfo {
            creator: CREATOR.to_string(),
            description: COLLECTION_DESCRIPTION.to_string(),
            image: COLLECTION_IMAGE.to_string(),
            external_link: None,
            explicit_content: None,
            start_trading_time: None,
            royalty_info: None,
        },
    };
    instantiate(deps.as_mut(), env.clone(), info, init_msg).unwrap();

    (deps, env)
}

pub fn mock_minter_info() -> MessageInfo {
    mock_info(MINTER, &[])
}

pub fn mock_creator_info() -> MessageInfo {
    mock_info(CREATOR, &[])
}

pub fn create_color(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}

pub fn create_position(x: u32, y: u32) -> Position {
    Position { x, y }
} 