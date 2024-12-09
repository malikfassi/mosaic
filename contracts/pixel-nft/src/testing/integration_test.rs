#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::*;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use cw_multi_test::Executor;

    #[test]
    fn test_full_flow() {
        let mut app = mock_app();

        // Set up contracts
        let (factory_addr, nft_addr, coloring_addr) = setup_contracts(&mut app);

        // Fund user account
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: "user".to_string(),
                amount: vec![Coin {
                    denom: "ustars".to_string(),
                    amount: Uint128::new(10_000_000),
                }],
            },
        ))
        .unwrap();

        // Mint pixel
        mint_pixel(&mut app, &factory_addr, "user", 0, 0).unwrap();

        // Verify NFT ownership
        let owner: sg721_base::msg::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                &nft_addr,
                &sg721_base::msg::QueryMsg::OwnerOf {
                    token_id: "0:0".to_string(),
                    include_expired: None,
                },
            )
            .unwrap();
        assert_eq!(owner.owner, Addr::unchecked("user"));

        // Set pixel color
        set_pixel_color(&mut app, &coloring_addr, "user", 0, 0, "#FF0000").unwrap();

        // Verify color change
        let color: Option<pixel_coloring::state::ColorChange> = app
            .wrap()
            .query_wasm_smart(
                &coloring_addr,
                &pixel_coloring::msg::QueryMsg::GetPixelColor { x: 0, y: 0 },
            )
            .unwrap();
        assert!(color.is_some());
        assert_eq!(color.unwrap().color, "#FF0000");

        // Verify NFT metadata update
        let nft: sg721_base::msg::NftInfoResponse<sg_metadata::Metadata> = app
            .wrap()
            .query_wasm_smart(
                &nft_addr,
                &sg721_base::msg::QueryMsg::NftInfo {
                    token_id: "0:0".to_string(),
                },
            )
            .unwrap();
        let color_trait = nft
            .extension
            .attributes
            .unwrap()
            .iter()
            .find(|t| t.trait_type == "color")
            .unwrap();
        assert_eq!(color_trait.value, "#FF0000");
    }

    #[test]
    fn test_unauthorized_color_change() {
        let mut app = mock_app();
        let (factory_addr, _nft_addr, coloring_addr) = setup_contracts(&mut app);

        // Fund both user accounts
        for user in ["user1", "user2"] {
            app.sudo(cw_multi_test::SudoMsg::Bank(
                cw_multi_test::BankSudo::Mint {
                    to_address: user.to_string(),
                    amount: vec![Coin {
                        denom: "ustars".to_string(),
                        amount: Uint128::new(10_000_000),
                    }],
                },
            ))
            .unwrap();
        }

        // User1 mints pixel
        mint_pixel(&mut app, &factory_addr, "user1", 0, 0).unwrap();

        // User2 tries to change color
        let err = set_pixel_color(&mut app, &coloring_addr, "user2", 0, 0, "#FF0000").unwrap_err();
        assert!(err.to_string().contains("Not the pixel owner"));
    }

    #[test]
    fn test_config_updates() {
        let mut app = mock_app();
        let (factory_addr, _nft_addr, coloring_addr) = setup_contracts(&mut app);

        // Update factory config
        app.execute_contract(
            Addr::unchecked("owner"),
            factory_addr.clone(),
            &ExecuteMsg::UpdateConfig {
                pixel_price: Some(2000000),
                color_change_price: Some(1000000),
                color_change_cooldown: Some(7200),
            },
            &[],
        )
        .unwrap();

        // Verify factory config update
        let config: Config = app
            .wrap()
            .query_wasm_smart(&factory_addr, &QueryMsg::GetConfig {})
            .unwrap();
        assert_eq!(config.pixel_price, Uint128::from(2000000u128));
        assert_eq!(config.color_change_price, Uint128::from(1000000u128));
        assert_eq!(config.color_change_cooldown, 7200);

        // Update coloring config
        app.execute_contract(
            Addr::unchecked("owner"),
            coloring_addr.clone(),
            &pixel_coloring::msg::ExecuteMsg::UpdateConfig {
                price_per_color_change: Some(Uint128::from(1000000u128)),
                color_change_cooldown: Some(7200),
            },
            &[],
        )
        .unwrap();

        // Verify coloring config update
        let config: pixel_coloring::state::Config = app
            .wrap()
            .query_wasm_smart(&coloring_addr, &pixel_coloring::msg::QueryMsg::GetConfig {})
            .unwrap();
        assert_eq!(config.price_per_color_change, Uint128::from(1000000u128));
        assert_eq!(config.color_change_cooldown, 7200);
    }
} 