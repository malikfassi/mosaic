use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Trait>,
}

#[cw_serde]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
pub struct PixelData {
    pub x: u32,
    pub y: u32,
    pub color: String,
} 