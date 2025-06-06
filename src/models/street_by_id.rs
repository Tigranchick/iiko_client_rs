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

/// StreetById : Street by id
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreetById {
    /// Street id.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Street classifierId.
    #[serde(
        rename = "classifierId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub classifier_id: Option<Option<String>>,
    /// City id.
    #[serde(rename = "cityId")]
    pub city_id: uuid::Uuid,
    /// City name.
    #[serde(rename = "cityName")]
    pub city_name: String,
    /// Street name.
    #[serde(rename = "streetName")]
    pub street_name: String,
}

impl StreetById {
    /// Street by id
    pub fn new(
        id: uuid::Uuid,
        city_id: uuid::Uuid,
        city_name: String,
        street_name: String,
    ) -> StreetById {
        StreetById {
            id,
            classifier_id: None,
            city_id,
            city_name,
            street_name,
        }
    }
}
