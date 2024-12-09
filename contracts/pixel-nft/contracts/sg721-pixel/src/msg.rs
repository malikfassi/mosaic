use cosmwasm_schema::cw_serde;
use sg721_base::msg::ExecuteMsg as Sg721ExecuteMsg;
use sg_metadata::Metadata;

#[cw_serde]
pub enum ExecuteMsg {
    Base(Sg721ExecuteMsg),
    UpdateMetadata {
        token_id: String,
        token_uri: Option<String>,
        extension: Option<Metadata>,
    },
} 