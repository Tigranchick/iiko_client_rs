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

/// ReserveInfo : Banquet/reserve.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReserveInfo {
    /// Banquet/reserve ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Banquet/reserve external number.
    #[serde(
        rename = "externalNumber",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_number: Option<Option<String>>,
    /// Organization ID.                Can be obtained by `/api/1/organizations` operation.
    #[serde(rename = "organizationId")]
    pub organization_id: uuid::Uuid,
    /// Timestamp of most recent banquet/reserve change that took place on iikoTransport server.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Banquet/reserve creation status. In case of asynchronous creation, it allows to track the instance an banquet/reserve was validated/created in iikoFront.
    #[serde(rename = "creationStatus")]
    pub creation_status: models::OrderCreationStatus,
    /// Banquet/reserve creation error details.  > Required only if \"creationStatus\"=\"Error\".
    #[serde(
        rename = "errorInfo",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_info: Option<Option<Box<models::ErrorInfo>>>,
    /// Banquet/reserve is deleted.
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    /// Banquet/reserve.
    #[serde(
        rename = "reserve",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reserve: Option<Option<Box<models::Reserve>>>,
}

impl ReserveInfo {
    /// Banquet/reserve.
    pub fn new(
        id: uuid::Uuid,
        organization_id: uuid::Uuid,
        timestamp: i64,
        creation_status: models::OrderCreationStatus,
        is_deleted: bool,
    ) -> ReserveInfo {
        ReserveInfo {
            id,
            external_number: None,
            organization_id,
            timestamp,
            creation_status,
            error_info: None,
            is_deleted,
            reserve: None,
        }
    }
}
