mod integration_test;
mod prop_test;

use cosmwasm_std::{Addr, Coin, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

use crate::{
    execute, instantiate, query,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::Config,
};

pub fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("user"),
                vec![Coin {
                    denom: "ustars".to_string(),
                    amount: Uint128::new(1_000_000_000),
                }],
            )
            .unwrap();
    })
}

pub fn factory_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

pub fn sg721_pixel_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        sg721_pixel::execute,
        sg721_pixel::instantiate,
        sg721_pixel::query,
    );
    Box::new(contract)
}

pub fn coloring_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        pixel_coloring::execute,
        pixel_coloring::instantiate,
        pixel_coloring::query,
    );
    Box::new(contract)
}

pub fn setup_contracts(app: &mut App) -> (Addr, Addr, Addr) {
    // Store contracts
    let factory_code_id = app.store_code(factory_contract());
    let nft_code_id = app.store_code(sg721_pixel_contract());
    let coloring_code_id = app.store_code(coloring_contract());

    // Instantiate factory
    let factory_addr = app
        .instantiate_contract(
            factory_code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                name: "Pixel NFTs".to_string(),
                symbol: "PIXEL".to_string(),
                canvas_width: 1000,
                canvas_height: 1000,
                pixel_price: Uint128::from(1000000u128),
                color_change_price: Uint128::from(500000u128),
                color_change_cooldown: 3600,
                nft_code_id,
                coloring_code_id,
                collection_image: "ipfs://...".to_string(),
            },
            &[],
            "Pixel Factory",
            None,
        )
        .unwrap();

    // Query instantiated contract addresses
    let contracts: (Option<Addr>, Option<Addr>) = app
        .wrap()
        .query_wasm_smart(&factory_addr, &QueryMsg::GetContracts {})
        .unwrap();

    (
        factory_addr,
        contracts.0.unwrap(),
        contracts.1.unwrap(),
    )
}

pub fn mint_pixel(
    app: &mut App,
    factory_addr: &Addr,
    sender: &str,
    x: u32,
    y: u32,
) -> anyhow::Result<()> {
    app.execute_contract(
        Addr::unchecked(sender),
        factory_addr.clone(),
        &ExecuteMsg::MintPixel {
            x,
            y,
            owner: sender.to_string(),
        },
        &[Coin {
            denom: "ustars".to_string(),
            amount: Uint128::from(1000000u128),
        }],
    )?;
    Ok(())
}

pub fn set_pixel_color(
    app: &mut App,
    coloring_addr: &Addr,
    sender: &str,
    x: u32,
    y: u32,
    color: &str,
) -> anyhow::Result<()> {
    app.execute_contract(
        Addr::unchecked(sender),
        coloring_addr.clone(),
        &pixel_coloring::msg::ExecuteMsg::SetPixelColor {
            x,
            y,
            color: color.to_string(),
        },
        &[Coin {
            denom: "ustars".to_string(),
            amount: Uint128::from(500000u128),
        }],
    )?;
    Ok(())
} 