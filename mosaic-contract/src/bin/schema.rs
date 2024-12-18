use cosmwasm_schema::write_api;
use cw721_base::msg::InstantiateMsg;
use mosaic_contract::msg::ExecuteMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
    }
}
