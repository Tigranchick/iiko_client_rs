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

/// RequestCreateOrderIikoCardDiscount : Card discount/surcharge.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestCreateOrderIikoCardDiscount {
    /// Card program ID.
    #[serde(rename = "programId")]
    pub program_id: uuid::Uuid,
    /// Card program name.
    #[serde(rename = "programName")]
    pub program_name: String,
    /// Discount information for order items.
    #[serde(rename = "discountItems")]
    pub discount_items: Vec<models::RequestCreateOrderIikoCardDiscountItem>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl RequestCreateOrderIikoCardDiscount {
    /// Card discount/surcharge.
    pub fn new(
        program_id: uuid::Uuid,
        program_name: String,
        discount_items: Vec<models::RequestCreateOrderIikoCardDiscountItem>,
        r#type: String,
    ) -> RequestCreateOrderIikoCardDiscount {
        RequestCreateOrderIikoCardDiscount {
            program_id,
            program_name,
            discount_items,
            r#type,
        }
    }
}
