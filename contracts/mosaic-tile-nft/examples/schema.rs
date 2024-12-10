use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{
    export_schema, export_schema_with_title, remove_schemas, schema_for, write_api,
};

use cosmwasm_std::Empty;
pub use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cw721_base::msg::InstantiateMsg;
pub use cw721_base::MinterResponse;
use mosaic_tile_nft::msg::{ExecuteMsg, QueryMsg};
pub use sg721_base::msg::CollectionInfoResponse;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(CollectionInfoResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<Empty>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema_with_title(&schema_for!(TokensResponse), &out_dir, "AllTokensResponse");
    export_schema_with_title(
        &schema_for!(OperatorsResponse),
        &out_dir,
        "AllOperatorsResponse",
    );
    export_schema(&schema_for!(MinterResponse), &out_dir);
    export_schema(&schema_for!(ApprovalResponse), &out_dir);
    export_schema(&schema_for!(ApprovalsResponse), &out_dir);
    export_schema(&schema_for!(ContractInfoResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(NftInfoResponse<Empty>),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
}
