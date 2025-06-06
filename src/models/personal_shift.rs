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

/// PersonalShift : Employee personal shift info.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalShift {
    /// Employee ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Employee Role ID.
    #[serde(
        rename = "roleId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub role_id: Option<Option<uuid::Uuid>>,
    /// Personal shift state flag (true - shift is opened, false - shift is closed).
    #[serde(rename = "opened")]
    pub opened: bool,
    /// ID of the terminal group where the personal shift is opened/closed.
    #[serde(rename = "terminalGroupId")]
    pub terminal_group_id: uuid::Uuid,
}

impl PersonalShift {
    /// Employee personal shift info.
    pub fn new(id: uuid::Uuid, opened: bool, terminal_group_id: uuid::Uuid) -> PersonalShift {
        PersonalShift {
            id,
            role_id: None,
            opened,
            terminal_group_id,
        }
    }
}
