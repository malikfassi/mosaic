use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw721_base::msg::InstantiateMsg;
pub use cw721_base::MinterResponse;
use mosaic_tile_nft::msg::{ExecuteMsg, QueryMsg};
use mosaic_tile_nft::state::{Color, Position, TileMetadata};
pub use sg721_base::msg::CollectionInfoResponse;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Color), &out_dir);
    export_schema(&schema_for!(Position), &out_dir);
    export_schema(&schema_for!(TileMetadata), &out_dir);
}
