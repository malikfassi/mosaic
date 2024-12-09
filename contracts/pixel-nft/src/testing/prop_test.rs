use super::*;
use crate::testing::*;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_multi_test::Executor;
use proptest::prelude::*;

prop_compose! {
    fn arb_pixel_coords(max_width: u32, max_height: u32)
        (x in 0..max_width, y in 0..max_height) -> (u32, u32) {
        (x, y)
    }
}

prop_compose! {
    fn arb_color()
        (r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) -> String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

proptest! {
    #[test]
    fn test_mint_and_color_pixels(
        (x1, y1) in arb_pixel_coords(1000, 1000),
        (x2, y2) in arb_pixel_coords(1000, 1000),
        color1 in arb_color(),
        color2 in arb_color(),
    ) {
        let mut app = mock_app();
        let (factory_addr, nft_addr, coloring_addr) = setup_contracts(&mut app);

        // Fund user accounts
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

        // User1 mints and colors first pixel
        mint_pixel(&mut app, &factory_addr, "user1", x1, y1).unwrap();
        set_pixel_color(&mut app, &coloring_addr, "user1", x1, y1, &color1).unwrap();

        // User2 mints and colors second pixel
        mint_pixel(&mut app, &factory_addr, "user2", x2, y2).unwrap();
        set_pixel_color(&mut app, &coloring_addr, "user2", x2, y2, &color2).unwrap();

        // Verify first pixel
        let color: Option<pixel_coloring::state::ColorChange> = app
            .wrap()
            .query_wasm_smart(
                &coloring_addr,
                &pixel_coloring::msg::QueryMsg::GetPixelColor { x: x1, y: y1 },
            )
            .unwrap();
        prop_assert!(color.is_some());
        prop_assert_eq!(color.unwrap().color, color1);

        // Verify second pixel
        let color: Option<pixel_coloring::state::ColorChange> = app
            .wrap()
            .query_wasm_smart(
                &coloring_addr,
                &pixel_coloring::msg::QueryMsg::GetPixelColor { x: x2, y: y2 },
            )
            .unwrap();
        prop_assert!(color.is_some());
        prop_assert_eq!(color.unwrap().color, color2);

        // Verify NFT ownership
        let owner: sg721_base::msg::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                &nft_addr,
                &sg721_base::msg::QueryMsg::OwnerOf {
                    token_id: format!("{}:{}", x1, y1),
                    include_expired: None,
                },
            )
            .unwrap();
        prop_assert_eq!(owner.owner, Addr::unchecked("user1"));

        let owner: sg721_base::msg::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                &nft_addr,
                &sg721_base::msg::QueryMsg::OwnerOf {
                    token_id: format!("{}:{}", x2, y2),
                    include_expired: None,
                },
            )
            .unwrap();
        prop_assert_eq!(owner.owner, Addr::unchecked("user2"));
    }

    #[test]
    fn test_config_updates(
        pixel_price in 1_000_000u128..10_000_000u128,
        color_price in 500_000u128..5_000_000u128,
        cooldown in 1800u64..7200u64,
    ) {
        let mut app = mock_app();
        let (factory_addr, _nft_addr, coloring_addr) = setup_contracts(&mut app);

        // Update factory config
        app.execute_contract(
            Addr::unchecked("owner"),
            factory_addr.clone(),
            &ExecuteMsg::UpdateConfig {
                pixel_price: Some(pixel_price),
                color_change_price: Some(color_price),
                color_change_cooldown: Some(cooldown),
            },
            &[],
        )
        .unwrap();

        // Verify factory config update
        let config: Config = app
            .wrap()
            .query_wasm_smart(&factory_addr, &QueryMsg::GetConfig {})
            .unwrap();
        prop_assert_eq!(config.pixel_price, Uint128::from(pixel_price));
        prop_assert_eq!(config.color_change_price, Uint128::from(color_price));
        prop_assert_eq!(config.color_change_cooldown, cooldown);

        // Update coloring config
        app.execute_contract(
            Addr::unchecked("owner"),
            coloring_addr.clone(),
            &pixel_coloring::msg::ExecuteMsg::UpdateConfig {
                price_per_color_change: Some(Uint128::from(color_price)),
                color_change_cooldown: Some(cooldown),
            },
            &[],
        )
        .unwrap();

        // Verify coloring config update
        let config: pixel_coloring::state::Config = app
            .wrap()
            .query_wasm_smart(&coloring_addr, &pixel_coloring::msg::QueryMsg::GetConfig {})
            .unwrap();
        prop_assert_eq!(config.price_per_color_change, Uint128::from(color_price));
        prop_assert_eq!(config.color_change_cooldown, cooldown);
    }
} 