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

/// DeliveryOrderErrorWebHookEventInfo : WebHook notification about delivery order saving error.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryOrderErrorWebHookEventInfo {
    /// Event type.
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    /// Event date and time (UTC).
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// Organization ID.                Can be obtained by `/api/1/organizations` operation.
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<uuid::Uuid>,
    /// Operation ID.
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<uuid::Uuid>,
    /// Event details.
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::OrderInfo>>,
}

impl DeliveryOrderErrorWebHookEventInfo {
    /// WebHook notification about delivery order saving error.
    pub fn new() -> DeliveryOrderErrorWebHookEventInfo {
        DeliveryOrderErrorWebHookEventInfo {
            event_type: None,
            event_time: None,
            organization_id: None,
            correlation_id: None,
            event_info: None,
        }
    }
}
/// Event type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "DeliveryOrderError")]
    DeliveryOrderError,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::DeliveryOrderError
    }
}
