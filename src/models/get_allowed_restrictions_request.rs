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

/// GetAllowedRestrictionsRequest : Request to identify suitable terminal groups.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAllowedRestrictionsRequest {
    /// Organization ID. Deprecated, use \"organizationIds\".
    #[serde(
        rename = "organizationId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_id: Option<Option<uuid::Uuid>>,
    /// Organization IDs.                Can be obtained by `/api/1/organizations` operation.
    #[serde(rename = "organizationIds", skip_serializing_if = "Option::is_none")]
    pub organization_ids: Option<Vec<uuid::Uuid>>,
    /// Delivery address.
    #[serde(
        rename = "deliveryAddress",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_address: Option<Option<Box<models::RestrictionsAddress>>>,
    /// Order location.
    #[serde(
        rename = "orderLocation",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_location: Option<Option<Box<models::OrderLocation>>>,
    /// Order list.
    #[serde(rename = "orderItems", skip_serializing_if = "Option::is_none")]
    pub order_items: Option<Vec<models::RestrictionsOrderItem>>,
    /// Type of delivery service.
    #[serde(rename = "isCourierDelivery")]
    pub is_courier_delivery: bool,
    /// Delivery date (Local for delivery terminal).
    #[serde(
        rename = "deliveryDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_date: Option<Option<String>>,
    /// Sum.
    #[serde(
        rename = "deliverySum",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_sum: Option<Option<f64>>,
    /// Discounts sum.
    #[serde(
        rename = "discountSum",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub discount_sum: Option<Option<f64>>,
}

impl GetAllowedRestrictionsRequest {
    /// Request to identify suitable terminal groups.
    pub fn new(is_courier_delivery: bool) -> GetAllowedRestrictionsRequest {
        GetAllowedRestrictionsRequest {
            organization_id: None,
            organization_ids: None,
            delivery_address: None,
            order_location: None,
            order_items: None,
            is_courier_delivery,
            delivery_date: None,
            delivery_sum: None,
            discount_sum: None,
        }
    }
}
