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

/// OrderItemModifier : Order item modifier.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItemModifier {
    /// Item.
    #[serde(rename = "product")]
    pub product: Box<models::OrderProduct>,
    /// Quantity.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Whether quantity of modifier depends on quantity of item.
    #[serde(rename = "amountIndependentOfParentAmount")]
    pub amount_independent_of_parent_amount: bool,
    /// Group of modifiers (in case of a group modifier).
    #[serde(
        rename = "productGroup",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_group: Option<Option<Box<models::OrderProductGroup>>>,
    /// Price per item unit. Can be sent different from the price in RMS.
    #[serde(rename = "price")]
    pub price: f64,
    /// Whether price is predefined.
    #[serde(rename = "pricePredefined")]
    pub price_predefined: bool,
    /// Total amount per item including tax, discounts/surcharges.
    #[serde(rename = "resultSum")]
    pub result_sum: f64,
    /// Item deletion details. If specified, the item is deleted.
    #[serde(
        rename = "deleted",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deleted: Option<Option<Box<models::OrderItemDeletedInfo>>>,
    /// Unique identifier of the item in the order and for the whole system.
    #[serde(
        rename = "positionId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_id: Option<Option<uuid::Uuid>>,
    /// Default amount.
    #[serde(
        rename = "defaultAmount",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_amount: Option<Option<i32>>,
    /// Hide modifier in UI if \"amount\" equals \"defaultAmount\".
    #[serde(
        rename = "hideIfDefaultAmount",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_if_default_amount: Option<Option<bool>>,
    /// Tax rate.
    #[serde(
        rename = "taxPercent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_percent: Option<Option<f64>>,
    /// Free of charge amount.
    #[serde(
        rename = "freeOfChargeAmount",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub free_of_charge_amount: Option<Option<i32>>,
}

impl OrderItemModifier {
    /// Order item modifier.
    pub fn new(
        product: models::OrderProduct,
        amount: f64,
        amount_independent_of_parent_amount: bool,
        price: f64,
        price_predefined: bool,
        result_sum: f64,
    ) -> OrderItemModifier {
        OrderItemModifier {
            product: Box::new(product),
            amount,
            amount_independent_of_parent_amount,
            product_group: None,
            price,
            price_predefined,
            result_sum,
            deleted: None,
            position_id: None,
            default_amount: None,
            hide_if_default_amount: None,
            tax_percent: None,
            free_of_charge_amount: None,
        }
    }
}
