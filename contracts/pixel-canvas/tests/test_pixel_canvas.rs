use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{coins, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use pixel_canvas::error::ContractError;
use pixel_canvas::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use pixel_canvas::state::{Config, Pixel, CONFIG, PIXELS};

const CREATOR: &str = "creator";
const USER1: &str = "user1";
const USER2: &str = "user2";
const NATIVE_DENOM: &str = "ustars";
const PIXEL_PRICE: u128 = 1_000_000; // 1 STARS
const CANVAS_SIZE: u32 = 100;

fn mock_app() -> App {
    App::default()
}

fn contract_pixel_canvas() -> Box<dyn Contract<String>> {
    let contract = ContractWrapper::new(
        pixel_canvas::contract::execute,
        pixel_canvas::contract::instantiate,
        pixel_canvas::contract::query,
    );
    Box::new(contract)
}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(CREATOR, &[]);

    // Initialize contract
    let msg = InstantiateMsg {
        canvas_size: CANVAS_SIZE,
        pixel_price: PIXEL_PRICE,
    };

    let res = pixel_canvas::contract::instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check config
    let config: Config = CONFIG.load(deps.as_ref().storage).unwrap();
    assert_eq!(config.owner, Addr::unchecked(CREATOR));
    assert_eq!(config.canvas_size, CANVAS_SIZE);
    assert_eq!(config.pixel_price, PIXEL_PRICE);
}

#[test]
fn buy_pixel() {
    let mut app = mock_app();
    let pixel_canvas_id = app.store_code(contract_pixel_canvas());

    // Instantiate contract
    let msg = InstantiateMsg {
        canvas_size: CANVAS_SIZE,
        pixel_price: PIXEL_PRICE,
    };
    let contract_addr = app
        .instantiate_contract(
            pixel_canvas_id,
            Addr::unchecked(CREATOR),
            &msg,
            &[],
            "pixel_canvas",
            None,
        )
        .unwrap();

    // Try to buy pixel without funds
    let msg = ExecuteMsg::BuyPixel { x: 5, y: 5 };
    let err = app
        .execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[])
        .unwrap_err();
    assert!(err.to_string().contains("insufficient funds"));

    // Buy pixel with correct funds
    let msg = ExecuteMsg::BuyPixel { x: 5, y: 5 };
    let funds = coins(PIXEL_PRICE, NATIVE_DENOM);
    app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &funds)
        .unwrap();

    // Query pixel
    let msg = QueryMsg::GetPixel { x: 5, y: 5 };
    let pixel: Option<Pixel> = app
        .wrap()
        .query_wasm_smart(&contract_addr, &msg)
        .unwrap();
    
    assert!(pixel.is_some());
    let pixel = pixel.unwrap();
    assert_eq!(pixel.owner, Addr::unchecked(USER1));
    assert_eq!(pixel.color, "#FFFFFF"); // Default color
}

#[test]
fn set_pixel_color() {
    let mut app = mock_app();
    let pixel_canvas_id = app.store_code(contract_pixel_canvas());

    // Instantiate contract
    let msg = InstantiateMsg {
        canvas_size: CANVAS_SIZE,
        pixel_price: PIXEL_PRICE,
    };
    let contract_addr = app
        .instantiate_contract(
            pixel_canvas_id,
            Addr::unchecked(CREATOR),
            &msg,
            &[],
            "pixel_canvas",
            None,
        )
        .unwrap();

    // Buy pixel
    let msg = ExecuteMsg::BuyPixel { x: 5, y: 5 };
    let funds = coins(PIXEL_PRICE, NATIVE_DENOM);
    app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &funds)
        .unwrap();

    // Try to set color as non-owner
    let msg = ExecuteMsg::SetPixelColor {
        x: 5,
        y: 5,
        color: "#FF0000".to_string(),
    };
    let err = app
        .execute_contract(Addr::unchecked(USER2), contract_addr.clone(), &msg, &[])
        .unwrap_err();
    assert!(err.to_string().contains("not pixel owner"));

    // Set color as owner
    let msg = ExecuteMsg::SetPixelColor {
        x: 5,
        y: 5,
        color: "#FF0000".to_string(),
    };
    app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[])
        .unwrap();

    // Query pixel
    let msg = QueryMsg::GetPixel { x: 5, y: 5 };
    let pixel: Option<Pixel> = app
        .wrap()
        .query_wasm_smart(&contract_addr, &msg)
        .unwrap();
    
    assert!(pixel.is_some());
    let pixel = pixel.unwrap();
    assert_eq!(pixel.color, "#FF0000");
}

#[test]
fn invalid_coordinates() {
    let mut app = mock_app();
    let pixel_canvas_id = app.store_code(contract_pixel_canvas());

    // Instantiate contract
    let msg = InstantiateMsg {
        canvas_size: CANVAS_SIZE,
        pixel_price: PIXEL_PRICE,
    };
    let contract_addr = app
        .instantiate_contract(
            pixel_canvas_id,
            Addr::unchecked(CREATOR),
            &msg,
            &[],
            "pixel_canvas",
            None,
        )
        .unwrap();

    // Try to buy pixel with invalid coordinates
    let msg = ExecuteMsg::BuyPixel {
        x: CANVAS_SIZE + 1,
        y: 5,
    };
    let funds = coins(PIXEL_PRICE, NATIVE_DENOM);
    let err = app
        .execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &funds)
        .unwrap_err();
    assert!(err.to_string().contains("invalid pixel coordinates"));
} 