use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct PixelMetadata {
    pub x: u32,
    pub y: u32,
    pub color: String,
}

impl From<PixelMetadata> for sg_metadata::Metadata {
    fn from(pixel: PixelMetadata) -> Self {
        sg_metadata::Metadata {
            image: None,
            image_data: None,
            external_url: None,
            description: Some(format!("Pixel at ({}, {})", pixel.x, pixel.y)),
            name: Some(format!("Pixel ({}, {})", pixel.x, pixel.y)),
            attributes: Some(vec![
                sg_metadata::Trait {
                    display_type: None,
                    trait_type: "x".to_string(),
                    value: pixel.x.to_string(),
                },
                sg_metadata::Trait {
                    display_type: None,
                    trait_type: "y".to_string(),
                    value: pixel.y.to_string(),
                },
                sg_metadata::Trait {
                    display_type: None,
                    trait_type: "color".to_string(),
                    value: pixel.color,
                },
            ]),
            background_color: None,
            animation_url: None,
            youtube_url: None,
        }
    }
} 