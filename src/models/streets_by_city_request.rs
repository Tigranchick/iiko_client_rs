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

/// StreetsByCityRequest : Organization and city request DTO.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreetsByCityRequest {
    /// Organization ID details of which have to be returned.                Can be obtained by `/api/1/organizations` operation.
    #[serde(rename = "organizationId")]
    pub organization_id: uuid::Uuid,
    /// City ID.
    #[serde(rename = "cityId")]
    pub city_id: uuid::Uuid,
    /// Include deleted streets in
    #[serde(rename = "includeDeleted", skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
}

impl StreetsByCityRequest {
    /// Organization and city request DTO.
    pub fn new(organization_id: uuid::Uuid, city_id: uuid::Uuid) -> StreetsByCityRequest {
        StreetsByCityRequest {
            organization_id,
            city_id,
            include_deleted: None,
        }
    }
}
