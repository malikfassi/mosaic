use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{from_binary, Event, attr};

use crate::{
    contract::{execute, query},
    msg::{ExecuteMsg, QueryMsg, TileInfoResponse},
    state::{ENABLE_UPDATABLE, FROZEN_TOKEN_METADATA},
    testing::{
        constants::{
            HACKER, OWNER, POSITION1, POSITION2, RED, GREEN, BLUE, TOKEN1, TOKEN2,
        },
        setup::{create_color, create_position, mock_minter_info, setup_contract},
    },
};

mod initialization {
    use super::*;

    #[test]
    fn proper_initialization() {
        let (deps, _) = setup_contract();
        
        // Check that metadata is not frozen initially
        let frozen = FROZEN_TOKEN_METADATA.load(deps.as_ref().storage).unwrap();
        assert!(!frozen, "Token metadata should not be frozen initially");

        // Check that updates are enabled initially
        let updatable = ENABLE_UPDATABLE.load(deps.as_ref().storage).unwrap();
        assert!(updatable, "Updates should be enabled initially");
    }
}

mod minting {
    use super::*;
    use crate::error::ContractError;

    #[test]
    fn successful_mint() {
        let (mut deps, env) = setup_contract();
        
        // Mint a tile
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION1.0, POSITION1.1),
            color: create_color(RED.0, RED.1, RED.2),
        };

        let res = execute(
            deps.as_mut(),
            env.clone(),
            mock_minter_info(),
            mint_msg,
        ).unwrap();

        // Verify response attributes
        assert_eq!(res.attributes.len(), 5, "Should have 5 attributes");
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "mint_tile"));
        assert!(res.attributes.iter().any(|attr| attr.key == "token_id" && attr.value == TOKEN1));

        // Verify color update event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "token_id" && attr.value == TOKEN1));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_r" && attr.value == RED.0.to_string()));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == RED.1.to_string()));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_b" && attr.value == RED.2.to_string()));

        // Query and verify tile info
        let query_msg = QueryMsg::TileInfo {
            token_id: TOKEN1.to_string(),
        };
        let res: TileInfoResponse = from_binary(&query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        
        assert_eq!(res.owner, OWNER);
        assert_eq!(res.metadata.position.x, POSITION1.0);
        assert_eq!(res.metadata.position.y, POSITION1.1);
        assert_eq!(res.metadata.current_color.r, RED.0);
        assert_eq!(res.metadata.current_color.g, RED.1);
        assert_eq!(res.metadata.current_color.b, RED.2);
    }

    #[test]
    fn unauthorized_mint() {
        let (mut deps, env) = setup_contract();
        
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION1.0, POSITION1.1),
            color: create_color(RED.0, RED.1, RED.2),
        };

        let unauthorized_info = mock_info(HACKER, &[]);
        let err = execute(
            deps.as_mut(),
            env,
            unauthorized_info,
            mint_msg,
        ).unwrap_err();

        assert!(matches!(err, ContractError::Unauthorized {}));
    }

    #[test]
    fn duplicate_position() {
        let (mut deps, env) = setup_contract();
        
        // First mint
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION1.0, POSITION1.1),
            color: create_color(RED.0, RED.1, RED.2),
        };
        execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();

        // Try to mint at same position
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN2.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION1.0, POSITION1.1),
            color: create_color(BLUE.0, BLUE.1, BLUE.2),
        };
        let err = execute(deps.as_mut(), env, mock_minter_info(), mint_msg).unwrap_err();

        assert!(matches!(err, ContractError::PositionTaken { x: POSITION1.0, y: POSITION1.1 }));
    }
}

mod color_updates {
    use super::*;
    use crate::error::ContractError;

    fn setup_tile(deps: &mut cosmwasm_std::OwnedDeps<cosmwasm_std::MemoryStorage, cosmwasm_std::testing::MockApi, cosmwasm_std::testing::MockQuerier, cosmwasm_std::Empty>, env: &cosmwasm_std::Env) {
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION1.0, POSITION1.1),
            color: create_color(RED.0, RED.1, RED.2),
        };
        execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();
    }

    #[test]
    fn successful_update() {
        let (mut deps, env) = setup_contract();
        setup_tile(&mut deps, &env);

        // Update color
        let update_msg = ExecuteMsg::UpdateTileColor {
            token_id: TOKEN1.to_string(),
            color: create_color(GREEN.0, GREEN.1, GREEN.2),
        };
        let owner_info = mock_info(OWNER, &[]);
        let res = execute(deps.as_mut(), env.clone(), owner_info, update_msg).unwrap();

        // Verify response
        assert_eq!(res.attributes.len(), 2); // action, token_id

        // Verify color update event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "token_id" && attr.value == TOKEN1));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_r" && attr.value == GREEN.0.to_string()));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == GREEN.1.to_string()));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_b" && attr.value == GREEN.2.to_string()));

        // Query updated state
        let query_msg = QueryMsg::TileInfo {
            token_id: TOKEN1.to_string(),
        };
        let res: TileInfoResponse = from_binary(&query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        
        assert_eq!(res.metadata.current_color.r, GREEN.0);
        assert_eq!(res.metadata.current_color.g, GREEN.1);
        assert_eq!(res.metadata.current_color.b, GREEN.2);
    }

    #[test]
    fn unauthorized_update() {
        let (mut deps, env) = setup_contract();
        setup_tile(&mut deps, &env);

        let update_msg = ExecuteMsg::UpdateTileColor {
            token_id: TOKEN1.to_string(),
            color: create_color(GREEN.0, GREEN.1, GREEN.2),
        };
        let hacker_info = mock_info(HACKER, &[]);
        let err = execute(deps.as_mut(), env, hacker_info, update_msg).unwrap_err();

        assert!(matches!(err, ContractError::Unauthorized {}));
    }

    #[test]
    fn update_frozen_metadata() {
        let (mut deps, env) = setup_contract();
        setup_tile(&mut deps, &env);

        // Freeze metadata
        execute(
            deps.as_mut(),
            env.clone(),
            mock_minter_info(),
            ExecuteMsg::FreezeTokenMetadata {},
        ).unwrap();

        // Try to update color
        let update_msg = ExecuteMsg::UpdateTileColor {
            token_id: TOKEN1.to_string(),
            color: create_color(GREEN.0, GREEN.1, GREEN.2),
        };
        let owner_info = mock_info(OWNER, &[]);
        let err = execute(deps.as_mut(), env, owner_info, update_msg).unwrap_err();

        assert!(matches!(err, ContractError::TokenMetadataFrozen {}));
    }
}

mod queries {
    use super::*;

    #[test]
    fn query_by_position() {
        let (mut deps, env) = setup_contract();
        
        // Mint tiles at different positions
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION1.0, POSITION1.1),
            color: create_color(RED.0, RED.1, RED.2),
        };
        execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();

        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN2.to_string(),
            owner: OWNER.to_string(),
            position: create_position(POSITION2.0, POSITION2.1),
            color: create_color(BLUE.0, BLUE.1, BLUE.2),
        };
        execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();

        // Query first position
        let query_msg = QueryMsg::TileAtPosition {
            position: create_position(POSITION1.0, POSITION1.1),
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let res: crate::msg::TileAtPositionResponse = from_binary(&res).unwrap();
        assert_eq!(res.token_id, Some(TOKEN1.to_string()));

        // Query second position
        let query_msg = QueryMsg::TileAtPosition {
            position: create_position(POSITION2.0, POSITION2.1),
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let res: crate::msg::TileAtPositionResponse = from_binary(&res).unwrap();
        assert_eq!(res.token_id, Some(TOKEN2.to_string()));

        // Query empty position
        let query_msg = QueryMsg::TileAtPosition {
            position: create_position(99, 99),
        };
        let res = query(deps.as_ref(), env, query_msg).unwrap();
        let res: crate::msg::TileAtPositionResponse = from_binary(&res).unwrap();
        assert_eq!(res.token_id, None);
    }
}

mod edge_cases {
    use super::*;
    use crate::state::MAX_POSITION;

    #[test]
    fn mint_at_boundary_positions() {
        let (mut deps, env) = setup_contract();
        
        // Test minting at (0, 0)
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(0, 0),
            color: create_color(RED.0, RED.1, RED.2),
        };
        let res = execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "mint_tile"));

        // Test minting at (MAX_POSITION, MAX_POSITION)
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN2.to_string(),
            owner: OWNER.to_string(),
            position: create_position(MAX_POSITION, MAX_POSITION),
            color: create_color(BLUE.0, BLUE.1, BLUE.2),
        };
        let res = execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "mint_tile"));

        // Test minting beyond MAX_POSITION
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN2.to_string(),
            owner: OWNER.to_string(),
            position: create_position(MAX_POSITION + 1, 0),
            color: create_color(GREEN.0, GREEN.1, GREEN.2),
        };
        let err = execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap_err();
        assert!(matches!(err, ContractError::InvalidPosition {}));
    }

    #[test]
    fn color_edge_cases() {
        let (mut deps, env) = setup_contract();
        
        // Test with extreme color values (0 and 255)
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(0, 0),
            color: create_color(0, 0, 0), // Black
        };
        let res = execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();
        
        // Verify black color event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_r" && attr.value == "0"));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == "0"));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_b" && attr.value == "0"));

        // Update to white
        let update_msg = ExecuteMsg::UpdateTileColor {
            token_id: TOKEN1.to_string(),
            color: create_color(255, 255, 255), // White
        };
        let owner_info = mock_info(OWNER, &[]);
        let res = execute(deps.as_mut(), env.clone(), owner_info, update_msg).unwrap();

        // Verify white color event
        let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_r" && attr.value == "255"));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == "255"));
        assert!(color_event.attributes.iter().any(|attr| attr.key == "color_b" && attr.value == "255"));
    }

    #[test]
    fn token_id_uniqueness() {
        let (mut deps, env) = setup_contract();
        
        // Mint first tile
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(0, 0),
            color: create_color(RED.0, RED.1, RED.2),
        };
        execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();

        // Try to mint with same token_id but different position
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(), // Same token_id
            owner: OWNER.to_string(),
            position: create_position(1, 1), // Different position
            color: create_color(BLUE.0, BLUE.1, BLUE.2),
        };
        let err = execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap_err();
        assert!(matches!(err, ContractError::Base(sg721_base::ContractError::Claimed {})));
    }

    #[test]
    fn multiple_color_updates() {
        let (mut deps, env) = setup_contract();
        
        // Mint a tile
        let mint_msg = ExecuteMsg::MintTile {
            token_id: TOKEN1.to_string(),
            owner: OWNER.to_string(),
            position: create_position(0, 0),
            color: create_color(RED.0, RED.1, RED.2),
        };
        execute(deps.as_mut(), env.clone(), mock_minter_info(), mint_msg).unwrap();

        // Perform multiple color updates
        let colors = [(255, 0, 0), (0, 255, 0), (0, 0, 255)];
        for (r, g, b) in colors.iter() {
            let update_msg = ExecuteMsg::UpdateTileColor {
                token_id: TOKEN1.to_string(),
                color: create_color(*r, *g, *b),
            };
            let owner_info = mock_info(OWNER, &[]);
            let res = execute(deps.as_mut(), env.clone(), owner_info, update_msg).unwrap();

            // Verify each color update event
            let color_event = res.events.iter().find(|e| e.ty == "tile_color_update").expect("Color update event not found");
            assert!(color_event.attributes.iter().any(|attr| attr.key == "color_r" && attr.value == r.to_string()));
            assert!(color_event.attributes.iter().any(|attr| attr.key == "color_g" && attr.value == g.to_string()));
            assert!(color_event.attributes.iter().any(|attr| attr.key == "color_b" && attr.value == b.to_string()));
        }

        // Verify final state
        let query_msg = QueryMsg::TileInfo {
            token_id: TOKEN1.to_string(),
        };
        let res: TileInfoResponse = from_binary(&query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        assert_eq!(res.metadata.current_color.r, 0);
        assert_eq!(res.metadata.current_color.g, 0);
        assert_eq!(res.metadata.current_color.b, 255);
    }

    #[test]
    fn metadata_state_transitions() {
        let (mut deps, env) = setup_contract();
        
        // Test freezing metadata
        execute(deps.as_mut(), env.clone(), mock_minter_info(), ExecuteMsg::FreezeTokenMetadata {}).unwrap();
        
        // Verify frozen state
        let frozen = FROZEN_TOKEN_METADATA.load(deps.as_ref().storage).unwrap();
        assert!(frozen);

        // Try to freeze again
        let err = execute(deps.as_mut(), env.clone(), mock_minter_info(), ExecuteMsg::FreezeTokenMetadata {}).unwrap_err();
        assert!(matches!(err, ContractError::TokenMetadataAlreadyFrozen {}));

        // Test enabling updatable when already enabled
        let err = execute(deps.as_mut(), env.clone(), mock_minter_info(), ExecuteMsg::EnableUpdatable {}).unwrap_err();
        assert!(matches!(err, ContractError::AlreadyEnableUpdatable {}));
    }
} 