#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, Addr, Empty, OwnedDeps, Response, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use mosaic_tile_nft::msg::{ExecuteMsg as NFTExecuteMsg, InstantiateMsg as NFTInstantiateMsg, QueryMsg as NFTQueryMsg};
    use mosaic_tile_nft::state::{Position, Color, TokenInfo};

    const OWNER: &str = "owner";
    const BUYER: &str = "buyer";
    const UNIT_PRICE: u128 = 1_000_000;

    fn mock_app() -> App {
        App::default()
    }

    fn store_nft_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(
            mosaic_tile_nft::contract::execute,
            mosaic_tile_nft::contract::instantiate,
            mosaic_tile_nft::contract::query,
        );
        app.store_code(Box::new(contract))
    }

    fn store_vending_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        app.store_code(Box::new(contract))
    }

    // Deploy NFT contract
    fn instantiate_nft(app: &mut App, nft_code_id: u64) -> String {
        app.instantiate_contract(
            nft_code_id,
            Addr::unchecked(OWNER),
            &NFTInstantiateMsg {
                name: "Mosaic Tiles".to_string(),
                symbol: "TILE".to_string(),
                minter: None, // Will be set to vending contract
            },
            &[],
            "Mosaic NFT",
            None,
        )
        .unwrap()
        .address
        .to_string()
    }

    // Deploy vending contract
    fn instantiate_vending(
        app: &mut App,
        vending_code_id: u64,
        nft_addr: String,
    ) -> String {
        app.instantiate_contract(
            vending_code_id,
            Addr::unchecked(OWNER),
            &InstantiateMsg {
                mosaic_nft_address: nft_addr,
                payment_address: OWNER.to_string(),
                unit_price: Uint128::from(UNIT_PRICE),
                max_batch_size: 10,
                random_minting_enabled: true,
                position_minting_enabled: true,
            },
            &[],
            "Mosaic Vending",
            None,
        )
        .unwrap()
        .address
        .to_string()
    }

    #[test]
    fn test_single_mint_flow() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let vending_code_id = store_vending_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let vending_addr = instantiate_vending(&mut app, vending_code_id, nft_addr.clone());

        // Set vending contract as minter
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(nft_addr.clone()),
            &NFTExecuteMsg::UpdateMinter {
                minter: Some(vending_addr.clone()),
            },
            &[],
        )
        .unwrap();

        // Mint a tile
        let position = Position { x: 5, y: 5 };
        let color = Color { r: 255, g: 0, b: 0 };
        let mint_msg = ExecuteMsg::MintPosition {
            position: position.clone(),
            color: color.clone(),
        };

        app.execute_contract(
            Addr::unchecked(BUYER),
            Addr::unchecked(vending_addr),
            &mint_msg,
            &coins(UNIT_PRICE, "ustars"),
        )
        .unwrap();

        // Query NFT contract to verify token
        let token_info: TokenInfo = app
            .wrap()
            .query_wasm_smart(
                nft_addr,
                &NFTQueryMsg::TokenInfo {
                    token_id: "tile_1".to_string(),
                },
            )
            .unwrap();

        assert_eq!(token_info.owner, BUYER.to_string());
        assert_eq!(token_info.position, position);
        assert_eq!(token_info.current_color, color);
    }

    #[test]
    fn test_batch_mint_flow() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let vending_code_id = store_vending_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let vending_addr = instantiate_vending(&mut app, vending_code_id, nft_addr.clone());

        // Set vending contract as minter
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(nft_addr.clone()),
            &NFTExecuteMsg::UpdateMinter {
                minter: Some(vending_addr.clone()),
            },
            &[],
        )
        .unwrap();

        // Batch mint tiles
        let mints = vec![
            (Position { x: 1, y: 1 }, Color { r: 255, g: 0, b: 0 }),
            (Position { x: 2, y: 2 }, Color { r: 0, g: 255, b: 0 }),
            (Position { x: 3, y: 3 }, Color { r: 0, g: 0, b: 255 }),
        ];
        let mint_msg = ExecuteMsg::BatchMintPositions { mints: mints.clone() };

        app.execute_contract(
            Addr::unchecked(BUYER),
            Addr::unchecked(vending_addr),
            &mint_msg,
            &coins(UNIT_PRICE * 3, "ustars"),
        )
        .unwrap();

        // Query NFT contract to verify all tokens
        for (i, (position, color)) in mints.iter().enumerate() {
            let token_info: TokenInfo = app
                .wrap()
                .query_wasm_smart(
                    nft_addr.clone(),
                    &NFTQueryMsg::TokenInfo {
                        token_id: format!("tile_{}", i + 1),
                    },
                )
                .unwrap();

            assert_eq!(token_info.owner, BUYER.to_string());
            assert_eq!(token_info.position, *position);
            assert_eq!(token_info.current_color, *color);
        }
    }

    #[test]
    fn test_random_mint_flow() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let vending_code_id = store_vending_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let vending_addr = instantiate_vending(&mut app, vending_code_id, nft_addr.clone());

        // Set vending contract as minter
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(nft_addr.clone()),
            &NFTExecuteMsg::UpdateMinter {
                minter: Some(vending_addr.clone()),
            },
            &[],
        )
        .unwrap();

        // Random mint
        let color = Color { r: 255, g: 0, b: 0 };
        let mint_msg = ExecuteMsg::MintRandom { color: color.clone() };

        let res = app
            .execute_contract(
                Addr::unchecked(BUYER),
                Addr::unchecked(vending_addr),
                &mint_msg,
                &coins(UNIT_PRICE, "ustars"),
            )
            .unwrap();

        // Query NFT contract to verify token
        let token_info: TokenInfo = app
            .wrap()
            .query_wasm_smart(
                nft_addr,
                &NFTQueryMsg::TokenInfo {
                    token_id: "tile_1".to_string(),
                },
            )
            .unwrap();

        assert_eq!(token_info.owner, BUYER.to_string());
        assert_eq!(token_info.current_color, color);
        // Position should be (0,0) since it's the first mint
        assert_eq!(token_info.position, Position { x: 0, y: 0 });
    }

    #[test]
    fn test_payment_handling() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let vending_code_id = store_vending_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let vending_addr = instantiate_vending(&mut app, vending_code_id, nft_addr.clone());

        // Set vending contract as minter
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(nft_addr.clone()),
            &NFTExecuteMsg::UpdateMinter {
                minter: Some(vending_addr.clone()),
            },
            &[],
        )
        .unwrap();

        // Test insufficient payment
        let mint_msg = ExecuteMsg::MintRandom {
            color: Color { r: 255, g: 0, b: 0 },
        };

        let err = app
            .execute_contract(
                Addr::unchecked(BUYER),
                Addr::unchecked(vending_addr.clone()),
                &mint_msg,
                &coins(UNIT_PRICE - 1, "ustars"),
            )
            .unwrap_err();

        assert!(err.to_string().contains("InsufficientPayment"));

        // Test wrong denom
        let err = app
            .execute_contract(
                Addr::unchecked(BUYER),
                Addr::unchecked(vending_addr),
                &mint_msg,
                &coins(UNIT_PRICE, "uatom"),
            )
            .unwrap_err();

        assert!(err.to_string().contains("Payment error"));
    }

    #[test]
    fn test_error_propagation() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let vending_code_id = store_vending_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let vending_addr = instantiate_vending(&mut app, vending_code_id, nft_addr);

        // Try to mint without setting minter
        let mint_msg = ExecuteMsg::MintRandom {
            color: Color { r: 255, g: 0, b: 0 },
        };

        let err = app
            .execute_contract(
                Addr::unchecked(BUYER),
                Addr::unchecked(vending_addr),
                &mint_msg,
                &coins(UNIT_PRICE, "ustars"),
            )
            .unwrap_err();

        assert!(err.to_string().contains("Unauthorized"));
    }
} 