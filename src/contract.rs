use cw721_base::state::TokenInfo;
use url::Url;

use cosmwasm_std::{
    to_json_binary, Addr, Binary, ContractInfoResponse, Decimal, Deps, DepsMut, Empty, Env, Event,
    MessageInfo, Response, StdError, StdResult, Storage, Timestamp, WasmQuery,
};

use cw721::{ContractInfoResponse as CW721ContractInfoResponse, Cw721Execute};
use cw_utils::nonpayable;
use serde::{de::DeserializeOwned, Serialize};
use cw_ownable::{assert_owner, get_ownership};

use sg721::{
    CollectionInfo, ExecuteMsg, InstantiateMsg, RoyaltyInfo, RoyaltyInfoResponse,
    UpdateCollectionInfoMsg,
};

use crate::msg::{CollectionInfoResponse, CustomExecuteMsg, QueryMsg};
use crate::{ContractError, Sg721Contract};
use crate::state::{
    Pixel, Tile, COLLECTION_INFO, FROZEN_COLLECTION_INFO, ROYALTY_UPDATED_AT, TILES,
};

// Constants
const MAX_DESCRIPTION_LENGTH: u32 = 512;
const MAX_SHARE_DELTA_PCT: u64 = 2;
const MAX_ROYALTY_SHARE_PCT: u64 = 10;
const PIXELS_PER_TILE: u32 = 100; // 10x10 grid

impl<'a, T> Sg721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // no funds should be sent to this contract
        nonpayable(&info)?;

        // check sender is a contract
        let req = WasmQuery::ContractInfo {
            contract_addr: info.sender.into(),
        }
        .into();
        let _res: ContractInfoResponse = deps
            .querier
            .query(&req)
            .map_err(|_| ContractError::Unauthorized {})?;

        // cw721 instantiation
        let info = CW721ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        self.parent.contract_info.save(deps.storage, &info)?;
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.minter))?;

        // sg721 instantiation
        if msg.collection_info.description.len() > MAX_DESCRIPTION_LENGTH as usize {
            return Err(ContractError::DescriptionTooLong {});
        }

        let image = Url::parse(&msg.collection_info.image)?;

        if let Some(ref external_link) = msg.collection_info.external_link {
            Url::parse(external_link)?;
        }

        let royalty_info: Option<RoyaltyInfo> = match msg.collection_info.royalty_info {
            Some(royalty_info) => Some(RoyaltyInfo {
                payment_address: deps.api.addr_validate(&royalty_info.payment_address)?,
                share: share_validate(royalty_info.share)?,
            }),
            None => None,
        };

        deps.api.addr_validate(&msg.collection_info.creator)?;

        let collection_info = CollectionInfo {
            creator: msg.collection_info.creator,
            description: msg.collection_info.description,
            image: msg.collection_info.image,
            external_link: msg.collection_info.external_link,
            explicit_content: msg.collection_info.explicit_content,
            start_trading_time: msg.collection_info.start_trading_time,
            royalty_info,
        };

        COLLECTION_INFO.save(deps.storage, &collection_info)?;
        FROZEN_COLLECTION_INFO.save(deps.storage, &false)?;
        ROYALTY_UPDATED_AT.save(deps.storage, &env.block.time)?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("collection_name", info.name)
            .add_attribute("collection_symbol", info.symbol)
            .add_attribute("collection_creator", collection_info.creator)
            .add_attribute("minter", msg.minter)
            .add_attribute("image", image.to_string()))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: sg721::ExecuteMsg<T, CustomExecuteMsg>,
    ) -> Result<Response, ContractError> {
        match msg {
            sg721::ExecuteMsg::Extension { msg } => match msg {
                CustomExecuteMsg::SetPixelColor {
                    pixel_id,
                    current_tile_metadata,
                    color,
                    expiration,
                } => self.execute_set_pixel_color(
                    deps,
                    env,
                    info,
                    pixel_id,
                    current_tile_metadata,
                    color,
                    expiration,
                ),
                CustomExecuteMsg::UpdateCollectionInfo { collection_info } => {
                    self.update_collection_info(deps, env, info, collection_info)
                }
                CustomExecuteMsg::UpdateStartTradingTime(start_time) => {
                    self.update_start_trading_time(deps, env, info, start_time)
                }
                CustomExecuteMsg::FreezeCollectionInfo {} => {
                    self.freeze_collection_info(deps, env, info)
                }
            },
            sg721::ExecuteMsg::UpdateCollectionInfo { collection_info } => {
                self.update_collection_info(deps, env, info, collection_info)
            }
            sg721::ExecuteMsg::UpdateStartTradingTime(start_time) => {
                self.update_start_trading_time(deps, env, info, start_time)
            }
            sg721::ExecuteMsg::FreezeCollectionInfo {} => {
                self.freeze_collection_info(deps, env, info)
            }
            sg721::ExecuteMsg::Burn { .. } => {
                Err(ContractError::FeatureDisabled { feature: "burn".to_string() })
            }
            _ => {
                // Convert our ExecuteMsg to cw721-base ExecuteMsg
                let base_msg = match msg {
                    sg721::ExecuteMsg::TransferNft { recipient, token_id } => {
                        cw721_base::ExecuteMsg::TransferNft { recipient, token_id }
                    }
                    sg721::ExecuteMsg::SendNft { contract, token_id, msg } => {
                        cw721_base::ExecuteMsg::SendNft { contract, token_id, msg }
                    }
                    sg721::ExecuteMsg::Approve { spender, token_id, expires } => {
                        cw721_base::ExecuteMsg::Approve { spender, token_id, expires }
                    }
                    sg721::ExecuteMsg::Revoke { spender, token_id } => {
                        cw721_base::ExecuteMsg::Revoke { spender, token_id }
                    }
                    sg721::ExecuteMsg::ApproveAll { operator, expires } => {
                        cw721_base::ExecuteMsg::ApproveAll { operator, expires }
                    }
                    sg721::ExecuteMsg::RevokeAll { operator } => {
                        cw721_base::ExecuteMsg::RevokeAll { operator }
                    }
                    sg721::ExecuteMsg::Mint { token_id, owner, token_uri, extension } => {
                        cw721_base::ExecuteMsg::Mint { token_id, owner, token_uri, extension }
                    }
                    _ => unreachable!("Other messages handled above"),
                };
                self.parent.execute(deps, env, info, base_msg).map_err(Into::into)
            }
        }
    }

    pub fn execute_set_pixel_color(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        pixel_id: u32,
        current_tile_metadata: Binary,
        color: [u8; 3],
        expiration: u64,
    ) -> Result<Response, ContractError> {
        // Validate pixel ID
        if pixel_id >= PIXELS_PER_TILE {
            return Err(ContractError::InvalidPixelId {});
        }

        // Validate expiration
        if expiration <= env.block.time.seconds() {
            return Err(ContractError::InvalidExpiration {});
        }

        // Calculate token ID from pixel ID
        let token_id = (pixel_id / PIXELS_PER_TILE).to_string();

        // Load tile
        let mut tile = TILES.may_load(deps.storage, token_id.clone())?
            .unwrap_or_else(|| Tile {
                owner: info.sender.clone(),
                pixels: vec![Pixel {
                    color: [0, 0, 0],
                    expiration: 0,
                }; PIXELS_PER_TILE as usize],
            });

        // Verify ownership
        if tile.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        // Check pixel availability
        let pixel_index = (pixel_id % PIXELS_PER_TILE) as usize;
        if tile.pixels[pixel_index].expiration > env.block.time.seconds() {
            return Err(ContractError::InvalidPixelUpdate("Pixel is not available".to_string()));
        }

        // Update pixel
        tile.pixels[pixel_index] = Pixel {
            color,
            expiration,
        };

        // Save tile
        TILES.save(deps.storage, token_id, &tile)?;

        Ok(Response::new()
            .add_attribute("action", "set_pixel_color")
            .add_attribute("pixel_id", pixel_id.to_string())
            .add_attribute("color", format!("{:?}", color))
            .add_attribute("expiration", expiration.to_string()))
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::CollectionInfo {} => to_json_binary(&self.query_collection_info(deps)?),
            QueryMsg::Ownership {} => self.parent.query(deps, env, cw721_base::QueryMsg::Ownership {}),
            _ => {
                // Convert sg721 QueryMsg to cw721-base QueryMsg
                let base_msg = match msg {
                    QueryMsg::OwnerOf { token_id, include_expired } => {
                        cw721_base::QueryMsg::OwnerOf { token_id, include_expired }
                    }
                    QueryMsg::Approval { token_id, spender, include_expired } => {
                        cw721_base::QueryMsg::Approval { token_id, spender, include_expired }
                    }
                    QueryMsg::Approvals { token_id, include_expired } => {
                        cw721_base::QueryMsg::Approvals { token_id, include_expired }
                    }
                    QueryMsg::AllOperators { owner, include_expired, start_after, limit } => {
                        cw721_base::QueryMsg::AllOperators { owner, include_expired, start_after, limit }
                    }
                    QueryMsg::NumTokens {} => cw721_base::QueryMsg::NumTokens {},
                    QueryMsg::ContractInfo {} => cw721_base::QueryMsg::ContractInfo {},
                    QueryMsg::NftInfo { token_id } => cw721_base::QueryMsg::NftInfo { token_id },
                    QueryMsg::AllNftInfo { token_id, include_expired } => {
                        cw721_base::QueryMsg::AllNftInfo { token_id, include_expired }
                    }
                    QueryMsg::Tokens { owner, start_after, limit } => {
                        cw721_base::QueryMsg::Tokens { owner, start_after, limit }
                    }
                    QueryMsg::AllTokens { start_after, limit } => {
                        cw721_base::QueryMsg::AllTokens { start_after, limit }
                    }
                    QueryMsg::Minter {} => cw721_base::QueryMsg::Minter {},
                    _ => unreachable!("Other messages handled above"),
                };
                self.parent.query(deps, env, base_msg)
            }
        }
    }

    pub fn update_collection_info(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        collection_msg: UpdateCollectionInfoMsg<RoyaltyInfoResponse>,
    ) -> Result<Response, ContractError> {
        let mut collection = COLLECTION_INFO.load(deps.storage)?;

        if FROZEN_COLLECTION_INFO.load(deps.storage)? {
            return Err(ContractError::CollectionInfoFrozen {});
        }

        // only creator can update collection info
        if collection.creator != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        if let Some(new_creator) = collection_msg.creator {
            deps.api.addr_validate(&new_creator)?;
            collection.creator = new_creator;
        }

        collection.description = collection_msg
            .description
            .unwrap_or_else(|| collection.description.to_string());
        if collection.description.len() > MAX_DESCRIPTION_LENGTH as usize {
            return Err(ContractError::DescriptionTooLong {});
        }

        collection.image = collection_msg
            .image
            .unwrap_or_else(|| collection.image.to_string());
        Url::parse(&collection.image)?;

        collection.external_link = collection_msg
            .external_link
            .unwrap_or_else(|| collection.external_link.as_ref().map(|s| s.to_string()));
        if collection.external_link.as_ref().is_some() {
            Url::parse(collection.external_link.as_ref().unwrap())?;
        }

        collection.explicit_content = collection_msg.explicit_content;

        if let Some(Some(new_royalty_info_response)) = collection_msg.royalty_info {
            let last_royalty_update = ROYALTY_UPDATED_AT.load(deps.storage)?;
            if last_royalty_update.plus_seconds(24 * 60 * 60) > env.block.time {
                return Err(ContractError::InvalidRoyalties(
                    "Royalties can only be updated once per day".to_string(),
                ));
            }

            let new_royalty_info = RoyaltyInfo {
                payment_address: deps
                    .api
                    .addr_validate(&new_royalty_info_response.payment_address)?,
                share: share_validate(new_royalty_info_response.share)?,
            };

            if let Some(old_royalty_info) = collection.royalty_info {
                if old_royalty_info.share < new_royalty_info.share {
                    let share_delta = new_royalty_info.share.abs_diff(old_royalty_info.share);

                    if share_delta > Decimal::percent(MAX_SHARE_DELTA_PCT) {
                        return Err(ContractError::InvalidRoyalties(format!(
                            "Share increase cannot be greater than {MAX_SHARE_DELTA_PCT}%"
                        )));
                    }
                    if new_royalty_info.share > Decimal::percent(MAX_ROYALTY_SHARE_PCT) {
                        return Err(ContractError::InvalidRoyalties(format!(
                            "Share cannot be greater than {MAX_ROYALTY_SHARE_PCT}%"
                        )));
                    }
                }
            }

            collection.royalty_info = Some(new_royalty_info);
            ROYALTY_UPDATED_AT.save(deps.storage, &env.block.time)?;
        }

        COLLECTION_INFO.save(deps.storage, &collection)?;

        let event = Event::new("update_collection_info").add_attribute("sender", info.sender);
        Ok(Response::new().add_event(event))
    }

    pub fn update_start_trading_time(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        start_time: Option<Timestamp>,
    ) -> Result<Response, ContractError> {
        assert_minter_owner(deps.storage, &info.sender)?;

        let mut collection_info = COLLECTION_INFO.load(deps.storage)?;
        collection_info.start_trading_time = start_time;
        COLLECTION_INFO.save(deps.storage, &collection_info)?;

        let event = Event::new("update_start_trading_time").add_attribute("sender", info.sender);
        Ok(Response::new().add_event(event))
    }

    pub fn freeze_collection_info(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let collection = self.query_collection_info(deps.as_ref())?;
        if collection.creator != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        let frozen = true;
        FROZEN_COLLECTION_INFO.save(deps.storage, &frozen)?;
        let event = Event::new("freeze_collection").add_attribute("sender", info.sender);
        Ok(Response::new().add_event(event))
    }

    pub fn query_collection_info(&self, deps: Deps) -> StdResult<CollectionInfoResponse> {
        let info = COLLECTION_INFO.load(deps.storage)?;

        let royalty_info_res: Option<RoyaltyInfoResponse> = match info.royalty_info {
            Some(royalty_info) => Some(RoyaltyInfoResponse {
                payment_address: royalty_info.payment_address.to_string(),
                share: royalty_info.share,
            }),
            None => None,
        };

        Ok(CollectionInfoResponse {
            creator: info.creator,
            description: info.description,
            image: info.image,
            external_link: info.external_link,
            explicit_content: info.explicit_content,
            start_trading_time: info.start_trading_time,
            royalty_info: royalty_info_res,
        })
    }
}

pub fn share_validate(share: Decimal) -> Result<Decimal, ContractError> {
    if share > Decimal::one() {
        return Err(ContractError::InvalidRoyalties(
            "Share cannot be greater than 100%".to_string(),
        ));
    }

    Ok(share)
}

pub fn get_owner_minter(storage: &mut dyn Storage) -> Result<Addr, ContractError> {
    let ownership = get_ownership(storage)?;
    match ownership.owner {
        Some(owner_value) => Ok(owner_value),
        None => Err(ContractError::MinterNotFound {}),
    }
}

pub fn assert_minter_owner(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let res = assert_owner(storage, sender);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(ContractError::UnauthorizedOwner {}),
    }
} 