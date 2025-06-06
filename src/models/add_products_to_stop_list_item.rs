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

/// AddProductsToStopListItem : Item for add to out-of-stock list.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddProductsToStopListItem {
    /// Out-of-stock list product ID.
    #[serde(rename = "productId")]
    pub product_id: uuid::Uuid,
    /// Out-of-stock list product size ID.
    #[serde(
        rename = "sizeId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub size_id: Option<Option<uuid::Uuid>>,
    /// Product balance.
    #[serde(rename = "balance")]
    pub balance: f64,
}

impl AddProductsToStopListItem {
    /// Item for add to out-of-stock list.
    pub fn new(product_id: uuid::Uuid, balance: f64) -> AddProductsToStopListItem {
        AddProductsToStopListItem {
            product_id,
            size_id: None,
            balance,
        }
    }
}
