use std::collections::HashMap;
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Binary, ContractResult, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, WasmQuery,
};
use mosaic_tile_nft::msg::QueryMsg as NFTQueryMsg;
use mosaic_tile_nft::state::{Position, Color};

use crate::testing::constants::*;

#[derive(Default)]
pub struct MockQuerier {
    token_owners: HashMap<String, String>,
    token_colors: HashMap<String, Color>,
}

impl MockQuerier {
    pub fn new() -> Self {
        Self {
            token_owners: HashMap::new(),
            token_colors: HashMap::new(),
        }
    }

    pub fn mock_nft_owner(&mut self, token_id: String, owner: String) {
        self.token_owners.insert(token_id, owner);
    }

    pub fn mock_nft_color(&mut self, token_id: String, color: Color) {
        self.token_colors.insert(token_id, color);
    }

    pub fn mock_nft_token(&mut self, position: Position, owner: String, color: Color) {
        let token_id = test_token_id(&position);
        self.mock_nft_owner(token_id.clone(), owner);
        self.mock_nft_color(token_id, color);
    }
}

impl Querier for MockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<_> = from_slice(bin_request).unwrap();
        match request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                if contract_addr == MOCK_NFT_CONTRACT {
                    self.handle_nft_query(&msg)
                } else {
                    SystemResult::Err(SystemError::InvalidRequest {
                        error: format!("Unknown contract: {}", contract_addr),
                        request: bin_request.into(),
                    })
                }
            }
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: "Unsupported query".to_string(),
                request: bin_request.into(),
            }),
        }
    }
}

impl MockQuerier {
    fn handle_nft_query(&self, msg: &Binary) -> QuerierResult {
        let query_msg: NFTQueryMsg = from_binary(msg).unwrap();
        match query_msg {
            NFTQueryMsg::OwnerOf { token_id, .. } => {
                let owner = self.token_owners.get(&token_id).cloned().unwrap_or_default();
                SystemResult::Ok(ContractResult::Ok(to_binary(&owner).unwrap()))
            }
            NFTQueryMsg::TokenInfo { token_id } => {
                let owner = self.token_owners.get(&token_id).cloned().unwrap_or_default();
                let color = self.token_colors.get(&token_id).cloned().unwrap_or_default();
                let parts: Vec<&str> = token_id.split('_').collect();
                let x: u32 = parts[1].parse().unwrap();
                let y: u32 = parts[2].parse().unwrap();
                let position = Position { x, y };

                SystemResult::Ok(ContractResult::Ok(to_binary(&mosaic_tile_nft::state::TokenInfo {
                    owner: owner,
                    position,
                    current_color: color,
                }).unwrap()))
            }
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: "Unsupported NFT query".to_string(),
                request: msg.to_vec(),
            }),
        }
    }
}

pub fn mock_dependencies() -> OwnedDeps<MockQuerier> {
    OwnedDeps::new(
        cosmwasm_std::testing::MockStorage::default(),
        cosmwasm_std::testing::MockApi::default(),
        MockQuerier::new(),
        cosmwasm_std::testing::MockRouter::default(),
    )
}

pub fn mock_dependencies_with_tokens(
    tokens: Vec<(Position, String, Color)>,
) -> OwnedDeps<MockQuerier> {
    let mut deps = mock_dependencies();
    for (position, owner, color) in tokens {
        deps.querier.mock_nft_token(position, owner, color);
    }
    deps
} 