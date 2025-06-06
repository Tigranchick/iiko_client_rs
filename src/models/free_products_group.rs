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

/// FreeProductsGroup : Free item to be added to order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FreeProductsGroup {
    /// Id of action that caused the suggestion.
    #[serde(rename = "sourceActionId", skip_serializing_if = "Option::is_none")]
    pub source_action_id: Option<uuid::Uuid>,
    /// Description for user. Can be null.
    #[serde(rename = "descriptionForUser", skip_serializing_if = "Option::is_none")]
    pub description_for_user: Option<String>,
    /// Products that should be added to order.
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<models::FreeProduct>>,
}

impl FreeProductsGroup {
    /// Free item to be added to order.
    pub fn new() -> FreeProductsGroup {
        FreeProductsGroup {
            source_action_id: None,
            description_for_user: None,
            products: None,
        }
    }
}
