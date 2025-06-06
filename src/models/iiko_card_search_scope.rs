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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IikoCardSearchScope {
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "Phone")]
    Phone,
    #[serde(rename = "CardNumber")]
    CardNumber,
    #[serde(rename = "CardTrack")]
    CardTrack,
    #[serde(rename = "PaymentToken")]
    PaymentToken,
    #[serde(rename = "FindFaceId")]
    FindFaceId,
}

impl std::fmt::Display for IikoCardSearchScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reserved => write!(f, "Reserved"),
            Self::Phone => write!(f, "Phone"),
            Self::CardNumber => write!(f, "CardNumber"),
            Self::CardTrack => write!(f, "CardTrack"),
            Self::PaymentToken => write!(f, "PaymentToken"),
            Self::FindFaceId => write!(f, "FindFaceId"),
        }
    }
}

impl Default for IikoCardSearchScope {
    fn default() -> IikoCardSearchScope {
        Self::Reserved
    }
}
