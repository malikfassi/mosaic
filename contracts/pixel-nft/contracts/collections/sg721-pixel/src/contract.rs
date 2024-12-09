use crate::error::ContractError;
use crate::state::{MAX_X, MAX_Y, PIXEL_COORDINATES, Sg721PixelContract};
use crate::PixelExtension;
use cosmwasm_std::{Deps, DepsMut, Empty, Env, MessageInfo, Response};
use cw721::Cw721Execute;
use sg721::ExecuteMsg;

impl<'a> Sg721PixelContract {
    pub fn mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        extension: PixelExtension,
    ) -> Result<Response, ContractError> {
        // Validate coordinates
        if extension.x >= MAX_X || extension.y >= MAX_Y {
            return Err(ContractError::InvalidPixelCoordinates {
                x: extension.x,
                y: extension.y,
            });
        }

        // Validate color format (hex color)
        if !extension.color.starts_with('#') || extension.color.len() != 7 {
            return Err(ContractError::InvalidColorFormat {
                color: extension.color,
            });
        }

        // Check if pixel already exists
        if PIXEL_COORDINATES.has(deps.storage, (extension.x, extension.y)) {
            return Err(ContractError::PixelAlreadyExists {
                x: extension.x,
                y: extension.y,
            });
        }

        // Store pixel coordinates
        PIXEL_COORDINATES.save(
            deps.storage,
            (extension.x, extension.y),
            &token_id,
        )?;

        // Call parent mint
        self.parent
            ._mint_token(deps, &env.block, &token_id, info.sender.as_ref(), Some(extension))?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("token_id", token_id)
            .add_attribute("x", extension.x.to_string())
            .add_attribute("y", extension.y.to_string())
            .add_attribute("color", extension.color))
    }

    pub fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response, ContractError> {
        // Call parent transfer
        self.parent
            .transfer_nft(deps, env, info, recipient.clone(), token_id.clone())?;

        Ok(Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("token_id", token_id)
            .add_attribute("recipient", recipient))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<PixelExtension, Empty>,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint {
                token_id,
                token_uri: _,
                extension,
            } => self.mint(deps, env, info, token_id, extension),
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => self.transfer_nft(deps, env, info, recipient, token_id),
            _ => Ok(self.parent.execute(deps, env, info, msg)?),
        }
    }
} 