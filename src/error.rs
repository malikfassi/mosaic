use cosmwasm_std::{StdError, Coin};
use cw_utils::PaymentError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Parse(#[from] ParseError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unauthorized Owner Does Not Match Sender")]
    UnauthorizedOwner {},

    #[error("Invalid pixel ID")]
    InvalidPixelId {},

    #[error("Pixel is out of range")]
    PixelOutOfRange {},

    #[error("Invalid pixel update: {0}")]
    InvalidPixelUpdate(String),

    #[error("Invalid fee. Expected {expected:?}, received {received:?}")]
    InvalidFee {
        expected: Coin,
        received: Option<Coin>,
    },

    #[error("Invalid expiration")]
    InvalidExpiration {},

    #[error("Base contract error: {0}")]
    Base(String),

    #[error("InvalidRoyalties: {0}")]
    InvalidRoyalties(String),

    #[error("Description too long")]
    DescriptionTooLong {},

    #[error("InvalidStartTradingTime")]
    InvalidStartTradingTime {},

    #[error("CollectionInfoFrozen")]
    CollectionInfoFrozen {},

    #[error("MinterNotFound")]
    MinterNotFound {},

    #[error("Ownership Update Error: {error}")]
    OwnershipUpdateError { error: String },

    #[error("Error while migrating: ({0}) ")]
    MigrationError(String),

    #[error("Feature disabled: {feature}")]
    FeatureDisabled { feature: String },
}

impl From<cw721_base::ContractError> for ContractError {
    fn from(err: cw721_base::ContractError) -> Self {
        ContractError::Base(err.to_string())
    }
} 