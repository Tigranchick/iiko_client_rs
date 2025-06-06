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

/// RequestCreateOrderExternalData : Order external data.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestCreateOrderExternalData {
    /// Key.
    #[serde(rename = "key")]
    pub key: String,
    /// Value.
    #[serde(rename = "value")]
    pub value: String,
    /// The transmitted data may contain both technical identifiers and information useful for the restaurant employee.  If it is necessary for the data to be included in the sales report, then this parameter must be set to TRUE, otherwise to FALSE.
    #[serde(rename = "isPublic", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

impl RequestCreateOrderExternalData {
    /// Order external data.
    pub fn new(key: String, value: String) -> RequestCreateOrderExternalData {
        RequestCreateOrderExternalData {
            key,
            value,
            is_public: None,
        }
    }
}
