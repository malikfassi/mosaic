use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use sg721_base::msg::{ExecuteMsg as Sg721BaseExecuteMsg, QueryMsg as Sg721BaseQueryMsg};
use sg721::{
    msg::{ExecuteMsg as Sg721ExecuteMsg, QueryMsg as Sg721QueryMsg},
    InstantiateMsg as Sg721InstantiateMsg,
};
use sg_metadata::Metadata;

pub type InstantiateMsg = Sg721InstantiateMsg;

#[cw_serde]
pub enum ExecuteMsg {
    Base(Sg721BaseExecuteMsg<Metadata, Empty>),
    UpdateMetadata {
        token_id: String,
        token_uri: Option<String>,
        extension: Option<Metadata>,
    },
}

pub type QueryMsg = Sg721BaseQueryMsg; 