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

/// PaymentLinkStatus : Статус платёжной ссылки на стороне транспорта.
/// Статус платёжной ссылки на стороне транспорта.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentLinkStatus {
    #[serde(rename = "CreationWaiting")]
    CreationWaiting,
    #[serde(rename = "New")]
    New,
    #[serde(rename = "ReadyToBeCaptured")]
    ReadyToBeCaptured,
    #[serde(rename = "Captured")]
    Captured,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "NotFound")]
    NotFound,
    #[serde(rename = "Refund")]
    Refund,
}

impl std::fmt::Display for PaymentLinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreationWaiting => write!(f, "CreationWaiting"),
            Self::New => write!(f, "New"),
            Self::ReadyToBeCaptured => write!(f, "ReadyToBeCaptured"),
            Self::Captured => write!(f, "Captured"),
            Self::Cancelled => write!(f, "Cancelled"),
            Self::NotFound => write!(f, "NotFound"),
            Self::Refund => write!(f, "Refund"),
        }
    }
}

impl Default for PaymentLinkStatus {
    fn default() -> PaymentLinkStatus {
        Self::CreationWaiting
    }
}
