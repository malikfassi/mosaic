use cosmwasm_std::{StdError, Coin};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

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
}

impl PartialEq for ContractError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ContractError::Std(a), ContractError::Std(b)) => a.to_string() == b.to_string(),
            (ContractError::Unauthorized {}, ContractError::Unauthorized {}) => true,
            (ContractError::InvalidPixelId {}, ContractError::InvalidPixelId {}) => true,
            (ContractError::PixelOutOfRange {}, ContractError::PixelOutOfRange {}) => true,
            (ContractError::InvalidPixelUpdate(a), ContractError::InvalidPixelUpdate(b)) => a == b,
            (ContractError::InvalidFee { expected: a_exp, received: a_rec }, 
             ContractError::InvalidFee { expected: b_exp, received: b_rec }) => {
                a_exp == b_exp && a_rec == b_rec
            },
            (ContractError::InvalidExpiration {}, ContractError::InvalidExpiration {}) => true,
            (ContractError::Base(a), ContractError::Base(b)) => a == b,
            _ => false,
        }
    }
}
