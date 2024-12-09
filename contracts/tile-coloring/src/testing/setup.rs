use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Addr, Env, MessageInfo, OwnedDeps, Response, Uint128,
};
use mosaic_tile_nft::state::{Position, Color};

use crate::{
    contract::{execute, instantiate},
    msg::{ExecuteMsg, InstantiateMsg},
    state::{Config, TilePermissions, UserStatistics},
    testing::{constants::*, mock_querier::MockQuerier},
};

pub struct TestContext {
    pub deps: OwnedDeps<MockQuerier>,
    pub env: Env,
    pub info: MessageInfo,
}

impl TestContext {
    pub fn new() -> Self {
        Self {
            deps: mock_dependencies(),
            env: mock_env(),
            info: mock_info(MOCK_ADMIN, &[]),
        }
    }

    pub fn with_tokens(tokens: Vec<(Position, String, Color)>) -> Self {
        Self {
            deps: mock_dependencies_with_tokens(tokens),
            env: mock_env(),
            info: mock_info(MOCK_ADMIN, &[]),
        }
    }

    pub fn instantiate_contract(&mut self) -> Result<Response, crate::error::ContractError> {
        let msg = InstantiateMsg {
            nft_contract: MOCK_NFT_CONTRACT.to_string(),
            admin: MOCK_ADMIN.to_string(),
            color_change_fee: default_color_change_fee(),
            rate_limit: DEFAULT_RATE_LIMIT,
            rate_limit_window: DEFAULT_RATE_LIMIT_WINDOW,
            requires_payment: false,
            rate_limiting_enabled: true,
        };
        instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
    }

    pub fn instantiate_contract_with_config(
        &mut self,
        color_change_fee: Uint128,
        rate_limit: u32,
        rate_limit_window: u64,
        requires_payment: bool,
        rate_limiting_enabled: bool,
    ) -> Result<Response, crate::error::ContractError> {
        let msg = InstantiateMsg {
            nft_contract: MOCK_NFT_CONTRACT.to_string(),
            admin: MOCK_ADMIN.to_string(),
            color_change_fee,
            rate_limit,
            rate_limit_window,
            requires_payment,
            rate_limiting_enabled,
        };
        instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
    }

    pub fn grant_permission(
        &mut self,
        sender: &str,
        position: Position,
        editor: &str,
    ) -> Result<Response, crate::error::ContractError> {
        let info = mock_info(sender, &[]);
        let msg = ExecuteMsg::GrantPermission {
            position,
            editor: editor.to_string(),
            expires_at: None,
        };
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    pub fn revoke_permission(
        &mut self,
        sender: &str,
        position: Position,
        editor: &str,
    ) -> Result<Response, crate::error::ContractError> {
        let info = mock_info(sender, &[]);
        let msg = ExecuteMsg::RevokePermission {
            position,
            editor: editor.to_string(),
        };
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    pub fn change_color(
        &mut self,
        sender: &str,
        position: Position,
        color: Color,
        funds: Vec<cosmwasm_std::Coin>,
    ) -> Result<Response, crate::error::ContractError> {
        let info = mock_info(sender, &funds);
        let msg = ExecuteMsg::ChangeColor { position, color };
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    pub fn set_public_editing(
        &mut self,
        sender: &str,
        position: Position,
        public_editing: bool,
        public_change_fee: Option<Uint128>,
    ) -> Result<Response, crate::error::ContractError> {
        let info = mock_info(sender, &[]);
        let msg = ExecuteMsg::SetPublicEditing {
            position,
            public_editing,
            public_change_fee,
        };
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    pub fn update_config(
        &mut self,
        sender: &str,
        nft_contract: Option<String>,
        admin: Option<String>,
        color_change_fee: Option<Uint128>,
        rate_limit: Option<u32>,
        rate_limit_window: Option<u64>,
        requires_payment: Option<bool>,
        rate_limiting_enabled: Option<bool>,
    ) -> Result<Response, crate::error::ContractError> {
        let info = mock_info(sender, &[]);
        let msg = ExecuteMsg::UpdateConfig {
            nft_contract,
            admin,
            color_change_fee,
            rate_limit,
            rate_limit_window,
            requires_payment,
            rate_limiting_enabled,
        };
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    pub fn advance_time(&mut self, seconds: u64) {
        self.env.block.time = self.env.block.time.plus_seconds(seconds);
    }
}

pub fn setup_test_case() -> TestContext {
    let mut ctx = TestContext::new();
    ctx.instantiate_contract().unwrap();
    ctx
}

pub fn setup_test_case_with_tokens(tokens: Vec<(Position, String, Color)>) -> TestContext {
    let mut ctx = TestContext::with_tokens(tokens);
    ctx.instantiate_contract().unwrap();
    ctx
}

pub fn setup_test_case_with_config(
    color_change_fee: Uint128,
    rate_limit: u32,
    rate_limit_window: u64,
    requires_payment: bool,
    rate_limiting_enabled: bool,
) -> TestContext {
    let mut ctx = TestContext::new();
    ctx.instantiate_contract_with_config(
        color_change_fee,
        rate_limit,
        rate_limit_window,
        requires_payment,
        rate_limiting_enabled,
    )
    .unwrap();
    ctx
} 