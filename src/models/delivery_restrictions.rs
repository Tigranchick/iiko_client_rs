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

/// DeliveryRestrictions : Delivery restrictions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRestrictions {
    /// Organization ID.                Can be obtained by `/api/1/organizations` operation.
    #[serde(rename = "organizationId")]
    pub organization_id: uuid::Uuid,
    /// Geocoding service type.
    #[serde(rename = "deliveryGeocodeServiceType")]
    pub delivery_geocode_service_type: models::DeliveryGeocodeServiceType,
    /// Link to the map of delivery service regions.
    #[serde(
        rename = "deliveryRegionsMapUrl",
        deserialize_with = "Option::deserialize"
    )]
    pub delivery_regions_map_url: Option<String>,
    /// General standard of delivery time.
    #[serde(rename = "defaultDeliveryDurationInMinutes")]
    pub default_delivery_duration_in_minutes: i64,
    /// Default pickup time.
    #[serde(rename = "defaultSelfServiceDurationInMinutes")]
    pub default_self_service_duration_in_minutes: i64,
    /// Indication that all delivery points in all delivery zones use common delivery time limits.
    #[serde(rename = "useSameDeliveryDuration")]
    pub use_same_delivery_duration: bool,
    /// Indication that all delivery points for all delivery zones use the total minimum order amount.
    #[serde(rename = "useSameMinSum")]
    pub use_same_min_sum: bool,
    /// Total minimum order amount.
    #[serde(rename = "defaultMinSum", deserialize_with = "Option::deserialize")]
    pub default_min_sum: Option<f64>,
    /// Indication that all delivery points in all zones use common time limits.
    #[serde(rename = "useSameWorkTimeInterval")]
    pub use_same_work_time_interval: bool,
    /// The beginning of the interval of the total work time for all points and delivery zones,   in minutes from the beginning of the day.
    #[serde(rename = "defaultFrom", deserialize_with = "Option::deserialize")]
    pub default_from: Option<i32>,
    /// End of the total work time interval for all points and delivery zones,   in minutes from the beginning of the day.
    #[serde(rename = "defaultTo", deserialize_with = "Option::deserialize")]
    pub default_to: Option<i32>,
    /// Indication that all delivery points in all zones use the same schedule for all days of the week.
    #[serde(rename = "useSameRestrictionsOnAllWeek")]
    pub use_same_restrictions_on_all_week: bool,
    /// Restrictions.
    #[serde(rename = "restrictions")]
    pub restrictions: Vec<models::DeliveryRestrictionItem>,
    /// Delivery zones.
    #[serde(rename = "deliveryZones")]
    pub delivery_zones: Vec<models::DeliveryZone>,
    /// Reject delivery if we could not geocode the address.
    #[serde(rename = "rejectOnGeocodingError")]
    pub reject_on_geocoding_error: bool,
    /// Add shipping cost to order.
    #[serde(rename = "addDeliveryServiceCost")]
    pub add_delivery_service_cost: bool,
    /// Indication that the cost is the same for all points of delivery.
    #[serde(rename = "useSameDeliveryServiceProduct")]
    pub use_same_delivery_service_product: bool,
    /// Link to \"delivery service payment\".
    #[serde(
        rename = "defaultDeliveryServiceProductId",
        deserialize_with = "Option::deserialize"
    )]
    pub default_delivery_service_product_id: Option<uuid::Uuid>,
    /// Use external delivery distribution service.
    #[serde(rename = "useExternalAssignationService")]
    pub use_external_assignation_service: bool,
    /// Indication whether or not to trust on the fronts the call center mapping restrictions from the call center  if the composition of the order has not changed since the last check. If true, then trust.
    #[serde(rename = "frontTrustsCallCenterCheck")]
    pub front_trusts_call_center_check: bool,
    /// Address of external delivery distribution service.
    #[serde(
        rename = "externalAssignationServiceUrl",
        deserialize_with = "Option::deserialize"
    )]
    pub external_assignation_service_url: Option<String>,
    /// Require an exact geocoding address.
    #[serde(rename = "requireExactAddressForGeocoding")]
    pub require_exact_address_for_geocoding: bool,
    /// Delivery restrictions mode.
    #[serde(rename = "zonesMode")]
    pub zones_mode: models::DeliveryRestrictionsMode,
    /// Automatically assigned delivery method based on cartography.
    #[serde(rename = "autoAssignExternalDeliveries")]
    pub auto_assign_external_deliveries: bool,
    /// Action on problems with auto-assignment.
    #[serde(rename = "actionOnValidationRejection")]
    pub action_on_validation_rejection: models::ActionOnValidationRejection,
}

impl DeliveryRestrictions {
    /// Delivery restrictions.
    pub fn new(
        organization_id: uuid::Uuid,
        delivery_geocode_service_type: models::DeliveryGeocodeServiceType,
        delivery_regions_map_url: Option<String>,
        default_delivery_duration_in_minutes: i64,
        default_self_service_duration_in_minutes: i64,
        use_same_delivery_duration: bool,
        use_same_min_sum: bool,
        default_min_sum: Option<f64>,
        use_same_work_time_interval: bool,
        default_from: Option<i32>,
        default_to: Option<i32>,
        use_same_restrictions_on_all_week: bool,
        restrictions: Vec<models::DeliveryRestrictionItem>,
        delivery_zones: Vec<models::DeliveryZone>,
        reject_on_geocoding_error: bool,
        add_delivery_service_cost: bool,
        use_same_delivery_service_product: bool,
        default_delivery_service_product_id: Option<uuid::Uuid>,
        use_external_assignation_service: bool,
        front_trusts_call_center_check: bool,
        external_assignation_service_url: Option<String>,
        require_exact_address_for_geocoding: bool,
        zones_mode: models::DeliveryRestrictionsMode,
        auto_assign_external_deliveries: bool,
        action_on_validation_rejection: models::ActionOnValidationRejection,
    ) -> DeliveryRestrictions {
        DeliveryRestrictions {
            organization_id,
            delivery_geocode_service_type,
            delivery_regions_map_url,
            default_delivery_duration_in_minutes,
            default_self_service_duration_in_minutes,
            use_same_delivery_duration,
            use_same_min_sum,
            default_min_sum,
            use_same_work_time_interval,
            default_from,
            default_to,
            use_same_restrictions_on_all_week,
            restrictions,
            delivery_zones,
            reject_on_geocoding_error,
            add_delivery_service_cost,
            use_same_delivery_service_product,
            default_delivery_service_product_id,
            use_external_assignation_service,
            front_trusts_call_center_check,
            external_assignation_service_url,
            require_exact_address_for_geocoding,
            zones_mode,
            auto_assign_external_deliveries,
            action_on_validation_rejection,
        }
    }
}
