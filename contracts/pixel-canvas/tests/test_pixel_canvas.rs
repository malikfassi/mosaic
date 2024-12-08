use cosmwasm_std::{
    Addr, Uint128, coins, from_json,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cw_multi_test::{App, ContractWrapper, Executor, AppBuilder};

use pixel_canvas::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use pixel_canvas::state::{Config, Pixel};

const CANVAS_WIDTH: u32 = 100;
const CANVAS_HEIGHT: u32 = 100;
const PIXEL_PRICE: u128 = 1000000;

fn contract_pixel_canvas() -> Box<dyn cw_multi_test::Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(
        pixel_canvas::execute,
        pixel_canvas::instantiate,
        pixel_canvas::query,
    );
    Box::new(contract)
}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        width: CANVAS_WIDTH,
        height: CANVAS_HEIGHT,
        price_per_pixel: Uint128::from(PIXEL_PRICE),
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = pixel_canvas::instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = pixel_canvas::query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let config: Config = from_json(&res).unwrap();
    assert_eq!(config.width, CANVAS_WIDTH);
    assert_eq!(config.height, CANVAS_HEIGHT);
    assert_eq!(config.price_per_pixel, Uint128::from(PIXEL_PRICE));
}

#[test]
fn buy_pixel() {
    let mut app = AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("buyer"),
                coins(PIXEL_PRICE, "ustars"),
            )
            .unwrap();
    });
    let pixel_canvas_id = app.store_code(contract_pixel_canvas());

    let msg = InstantiateMsg {
        width: CANVAS_WIDTH,
        height: CANVAS_HEIGHT,
        price_per_pixel: Uint128::from(PIXEL_PRICE),
    };

    let pixel_canvas_addr = app
        .instantiate_contract(
            pixel_canvas_id,
            Addr::unchecked("creator"),
            &msg,
            &[],
            "pixel-canvas",
            None,
        )
        .unwrap();

    // Buy a pixel
    let buy_msg = ExecuteMsg::BuyPixel { x: 0, y: 0 };
    let res = app
        .execute_contract(
            Addr::unchecked("buyer"),
            pixel_canvas_addr.clone(),
            &buy_msg,
            &coins(PIXEL_PRICE, "ustars"),
        )
        .unwrap();
    assert_eq!(2, res.events.len());

    // Query the pixel
    let pixel: Option<Pixel> = app
        .wrap()
        .query_wasm_smart(
            pixel_canvas_addr,
            &QueryMsg::GetPixel { x: 0, y: 0 },
        )
        .unwrap();
    let pixel = pixel.unwrap();
    assert_eq!(pixel.owner, Addr::unchecked("buyer"));
    assert_eq!(pixel.color, "#FFFFFF");
} 