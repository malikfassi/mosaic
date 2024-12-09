#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{coins, Addr, Empty, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use mosaic_tile_nft::msg::{ExecuteMsg as NFTExecuteMsg, InstantiateMsg as NFTInstantiateMsg};
    use mosaic_tile_nft::state::{Position, Color};

    const OWNER: &str = "owner";
    const USER1: &str = "user1";
    const USER2: &str = "user2";
    const ADMIN: &str = "admin";

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

    fn store_coloring_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        app.store_code(Box::new(contract))
    }

    fn instantiate_nft(app: &mut App, nft_code_id: u64) -> String {
        app.instantiate_contract(
            nft_code_id,
            Addr::unchecked(OWNER),
            &NFTInstantiateMsg {
                name: "Mosaic Tiles".to_string(),
                symbol: "TILE".to_string(),
                minter: None,
            },
            &[],
            "Mosaic NFT",
            None,
        )
        .unwrap()
        .address
        .to_string()
    }

    fn instantiate_coloring(
        app: &mut App,
        coloring_code_id: u64,
        nft_addr: String,
    ) -> String {
        app.instantiate_contract(
            coloring_code_id,
            Addr::unchecked(OWNER),
            &InstantiateMsg {
                nft_contract: nft_addr,
                admin: ADMIN.to_string(),
                color_change_fee: Uint128::zero(),
                rate_limit: 10,
                rate_limit_window: 3600,
                requires_payment: false,
                rate_limiting_enabled: true,
            },
            &[],
            "Tile Coloring",
            None,
        )
        .unwrap()
        .address
        .to_string()
    }

    fn mint_nft(
        app: &mut App,
        nft_addr: &str,
        owner: &str,
        position: Position,
        color: Color,
    ) -> String {
        let token_id = format!("tile_{}_{}",  position.x, position.y);
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(nft_addr),
            &NFTExecuteMsg::MintTile {
                token_id: token_id.clone(),
                owner: owner.to_string(),
                position,
                color,
            },
            &[],
        )
        .unwrap();
        token_id
    }

    #[test]
    fn test_permission_system() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let coloring_code_id = store_coloring_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let coloring_addr = instantiate_coloring(&mut app, coloring_code_id, nft_addr.clone());

        // Mint NFT for USER1
        let position = Position { x: 1, y: 1 };
        let color = Color { r: 255, g: 0, b: 0 };
        mint_nft(&mut app, &nft_addr, USER1, position.clone(), color.clone());

        // USER1 grants permission to USER2
        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::GrantPermission {
                position: position.clone(),
                editor: USER2.to_string(),
                expires_at: None,
            },
            &[],
        )
        .unwrap();

        // USER2 can change color
        let new_color = Color { r: 0, g: 255, b: 0 };
        app.execute_contract(
            Addr::unchecked(USER2),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::ChangeColor {
                position: position.clone(),
                color: new_color.clone(),
            },
            &[],
        )
        .unwrap();

        // USER1 revokes permission from USER2
        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::RevokePermission {
                position: position.clone(),
                editor: USER2.to_string(),
            },
            &[],
        )
        .unwrap();

        // USER2 can no longer change color
        let err = app
            .execute_contract(
                Addr::unchecked(USER2),
                Addr::unchecked(coloring_addr.clone()),
                &ExecuteMsg::ChangeColor {
                    position: position.clone(),
                    color: Color { r: 0, g: 0, b: 255 },
                },
                &[],
            )
            .unwrap_err();
        assert!(err.to_string().contains("Unauthorized"));
    }

    #[test]
    fn test_nft_transfer_handling() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let coloring_code_id = store_coloring_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let coloring_addr = instantiate_coloring(&mut app, coloring_code_id, nft_addr.clone());

        // Set coloring contract as minter
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(nft_addr.clone()),
            &NFTExecuteMsg::UpdateMinter {
                minter: Some(coloring_addr.clone()),
            },
            &[],
        )
        .unwrap();

        // Mint NFT for USER1
        let position = Position { x: 1, y: 1 };
        let color = Color { r: 255, g: 0, b: 0 };
        let token_id = mint_nft(&mut app, &nft_addr, USER1, position.clone(), color.clone());

        // USER1 grants permission to USER2
        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::GrantPermission {
                position: position.clone(),
                editor: USER2.to_string(),
                expires_at: None,
            },
            &[],
        )
        .unwrap();

        // Transfer NFT from USER1 to OWNER
        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(nft_addr.clone()),
            &NFTExecuteMsg::TransferNft {
                recipient: OWNER.to_string(),
                token_id: token_id.clone(),
            },
            &[],
        )
        .unwrap();

        // USER2's permission should be revoked
        let err = app
            .execute_contract(
                Addr::unchecked(USER2),
                Addr::unchecked(coloring_addr.clone()),
                &ExecuteMsg::ChangeColor {
                    position: position.clone(),
                    color: Color { r: 0, g: 255, b: 0 },
                },
                &[],
            )
            .unwrap_err();
        assert!(err.to_string().contains("Unauthorized"));

        // OWNER should be able to change color
        app.execute_contract(
            Addr::unchecked(OWNER),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::ChangeColor {
                position: position.clone(),
                color: Color { r: 0, g: 255, b: 0 },
            },
            &[],
        )
        .unwrap();
    }

    #[test]
    fn test_batch_operations() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let coloring_code_id = store_coloring_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let coloring_addr = instantiate_coloring(&mut app, coloring_code_id, nft_addr.clone());

        // Mint multiple NFTs for USER1
        let positions = vec![
            Position { x: 1, y: 1 },
            Position { x: 2, y: 2 },
            Position { x: 3, y: 3 },
        ];
        let color = Color { r: 255, g: 0, b: 0 };
        for pos in positions.iter() {
            mint_nft(&mut app, &nft_addr, USER1, pos.clone(), color.clone());
        }

        // Batch grant permissions to USER2
        let permissions: Vec<(Position, String, Option<Timestamp>)> = positions
            .iter()
            .map(|pos| (pos.clone(), USER2.to_string(), None))
            .collect();

        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::BatchGrantPermission { permissions },
            &[],
        )
        .unwrap();

        // USER2 can change colors for all positions
        for pos in positions.iter() {
            app.execute_contract(
                Addr::unchecked(USER2),
                Addr::unchecked(coloring_addr.clone()),
                &ExecuteMsg::ChangeColor {
                    position: pos.clone(),
                    color: Color { r: 0, g: 255, b: 0 },
                },
                &[],
            )
            .unwrap();
        }

        // Batch revoke permissions
        let revocations: Vec<(Position, String)> = positions
            .iter()
            .map(|pos| (pos.clone(), USER2.to_string()))
            .collect();

        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::BatchRevokePermission { permissions: revocations },
            &[],
        )
        .unwrap();

        // USER2 can no longer change colors
        for pos in positions.iter() {
            let err = app
                .execute_contract(
                    Addr::unchecked(USER2),
                    Addr::unchecked(coloring_addr.clone()),
                    &ExecuteMsg::ChangeColor {
                        position: pos.clone(),
                        color: Color { r: 0, g: 0, b: 255 },
                    },
                    &[],
                )
                .unwrap_err();
            assert!(err.to_string().contains("Unauthorized"));
        }
    }

    #[test]
    fn test_public_editing() {
        let mut app = mock_app();

        // Deploy contracts
        let nft_code_id = store_nft_code(&mut app);
        let coloring_code_id = store_coloring_code(&mut app);
        let nft_addr = instantiate_nft(&mut app, nft_code_id);
        let coloring_addr = instantiate_coloring(&mut app, coloring_code_id, nft_addr.clone());

        // Mint NFT for USER1
        let position = Position { x: 1, y: 1 };
        let color = Color { r: 255, g: 0, b: 0 };
        mint_nft(&mut app, &nft_addr, USER1, position.clone(), color.clone());

        // Enable public editing with fee
        let public_fee = Uint128::from(1000000u128);
        app.execute_contract(
            Addr::unchecked(USER1),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::SetPublicEditing {
                position: position.clone(),
                public_editing: true,
                public_change_fee: Some(public_fee),
            },
            &[],
        )
        .unwrap();

        // USER2 can change color with payment
        app.execute_contract(
            Addr::unchecked(USER2),
            Addr::unchecked(coloring_addr.clone()),
            &ExecuteMsg::ChangeColor {
                position: position.clone(),
                color: Color { r: 0, g: 255, b: 0 },
            },
            &coins(public_fee.u128(), "ustars"),
        )
        .unwrap();

        // USER2 cannot change color without payment
        let err = app
            .execute_contract(
                Addr::unchecked(USER2),
                Addr::unchecked(coloring_addr.clone()),
                &ExecuteMsg::ChangeColor {
                    position: position.clone(),
                    color: Color { r: 0, g: 0, b: 255 },
                },
                &[],
            )
            .unwrap_err();
        assert!(err.to_string().contains("InsufficientPayment"));
    }
} 