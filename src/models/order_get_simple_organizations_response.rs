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

/// OrderGetSimpleOrganizationsResponse : Response to request for organizations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderGetSimpleOrganizationsResponse {
    /// Operation ID.
    #[serde(rename = "correlationId")]
    pub correlation_id: uuid::Uuid,
    /// List of organizations.                Can be obtained by `/api/1/organizations` operation.
    #[serde(rename = "organizations")]
    pub organizations: Vec<models::OrderSimpleOrganizationInfo>,
}

impl OrderGetSimpleOrganizationsResponse {
    /// Response to request for organizations.
    pub fn new(
        correlation_id: uuid::Uuid,
        organizations: Vec<models::OrderSimpleOrganizationInfo>,
    ) -> OrderGetSimpleOrganizationsResponse {
        OrderGetSimpleOrganizationsResponse {
            correlation_id,
            organizations,
        }
    }
}
