use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty, Timestamp};
use cw_storage_plus::{Item, Map};
use serde::{de::DeserializeOwned, Serialize};
use sg721::{CollectionInfo, RoyaltyInfo};
use std::ops::Deref;

#[cw_serde]
pub struct Pixel {
    pub color: [u8; 3],
    pub expiration: u64,
}

#[cw_serde]
pub struct Tile {
    pub owner: Addr,
    pub pixels: Vec<Pixel>,
}

// Key is token_id
pub const TILES: Map<String, Tile> = Map::new("tiles");

// Collection info storage
pub const COLLECTION_INFO: Item<CollectionInfo<RoyaltyInfo>> = Item::new("collection_info");

// Flag to freeze collection info
pub const FROZEN_COLLECTION_INFO: Item<bool> = Item::new("frozen_collection_info");

// Track last royalty update time
pub const ROYALTY_UPDATED_AT: Item<Timestamp> = Item::new("royalty_updated_at");

// Parent contract wrapper
pub type Parent<'a, T> = cw721_base::Cw721Contract<'a, T, Empty, Empty, Empty>;

pub struct Sg721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub parent: Parent<'a, T>,
    pub collection_info: Item<'a, CollectionInfo<RoyaltyInfo>>,
    pub frozen_collection_info: Item<'a, bool>,
    pub royalty_updated_at: Item<'a, Timestamp>,
}

impl<'a, T> Default for Sg721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Sg721Contract {
            parent: Parent::default(),
            collection_info: Item::new("collection_info"),
            frozen_collection_info: Item::new("frozen_collection_info"),
            royalty_updated_at: Item::new("royalty_updated_at"),
        }
    }
}

impl<'a, T> Deref for Sg721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    type Target = Parent<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
} 