/*
 * iikoCloud API
 *
 * <h3>Description of common data formats:</h3><b>uuid</b> - string in UUID(universally unique identifier).<br/>Examples: <i>550e8400-e29b-41d4-a716-446655440000, b090de0b-8550-6e17-70b2-bbba152bcbd3</i><br/><br/><b>date-time</b> - date and time string in custom string format <b>yyyy-MM-dd HH:mm:ss.fff</b>.<br/>Examples: <i>2017-04-29 20:45:00.000, 2018-01-01 01:01:30.123</i>
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransportItemSizeDto {
    #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<models::TransportPriceDto>>,
    #[serde(rename = "itemModifierGroups", skip_serializing_if = "Option::is_none")]
    pub item_modifier_groups: Option<Vec<models::TransportModifierGroupDto>>,
    /// Unique size code, consists of the product code and the name of the size, if the product has one size, then the size code will be equal to the product code
    #[serde(rename = "sku", skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "sizeCode", skip_serializing_if = "Option::is_none")]
    pub size_code: Option<String>,
    /// Name of the product size, the name can be empty if there is only one size in the list
    #[serde(rename = "sizeName", skip_serializing_if = "Option::is_none")]
    pub size_name: Option<String>,
    /// Whether it is a default size of the product. If the product has one size, then the parameter will be true, if the product has several sizes, none of them can be default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Size's weight
    #[serde(rename = "portionWeightGrams", skip_serializing_if = "Option::is_none")]
    pub portion_weight_grams: Option<f32>,
    /// ID size, can be empty if the default size is selected and it is the only size in the list
    #[serde(rename = "sizeId", skip_serializing_if = "Option::is_none")]
    pub size_id: Option<uuid::Uuid>,
    #[serde(
        rename = "nutritionPerHundredGrams",
        skip_serializing_if = "Option::is_none"
    )]
    pub nutrition_per_hundred_grams: Option<serde_json::Value>,
    /// links to images
    #[serde(rename = "buttonImageUrl", skip_serializing_if = "Option::is_none")]
    pub button_image_url: Option<String>,
    #[serde(
        rename = "buttonImageCroppedUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub button_image_cropped_url: Option<Vec<String>>,
}

impl TransportItemSizeDto {
    pub fn new() -> TransportItemSizeDto {
        TransportItemSizeDto {
            prices: None,
            item_modifier_groups: None,
            sku: None,
            size_code: None,
            size_name: None,
            is_default: None,
            portion_weight_grams: None,
            size_id: None,
            nutrition_per_hundred_grams: None,
            button_image_url: None,
            button_image_cropped_url: None,
        }
    }
}
