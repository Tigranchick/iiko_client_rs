# OrderOrganizationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID. | 
**prices_vat_inclusive** | Option<**bool**> | Determines whether organization prices include VAT.                Available if `VAT` requested. | [optional]
**loyalty_discount_affects_vat** | Option<**bool**> | Determines whether organization loyalty discounts affects VAT.                > Working only if \"pricesVatInclusive\" = false                Available if `VAT` requested. | [optional]
**version** | Option<**String**> | RMS version.                Available if `Version` requested. | [optional]
**address_format_type** | Option<[**models::OrderAddressFormatType**](OrderAddressFormatType.md)> | Address format type.                Available if `AddressFormatType` requested. | [optional]
**is_anonymous_guests_allowed** | Option<**bool**> | If the store allows orders for anonymous guests, then it is not necessary to transfer  information about the guest as part of the delivery order. You can only transfer  the phone number and optionally name of the guest, which will not be stored in the guest base  and will only be used for the delivery of a current delivery order.                Available if `IsAnonymousGuestsAllowed` requested. | [optional]
**name** | Option<**String**> | Organization name.                Available if `Name` requested. | [optional]
**country** | Option<**String**> | Country.                Available if `Country` requested. | [optional]
**restaurant_address** | Option<**String**> | Restaurant address.                Available if `RestaurantAddress` requested. | [optional]
**latitude** | Option<**f64**> | Latitude.                Available if `Latitude` requested. | [optional]
**longitude** | Option<**f64**> | Longitude.                Available if `Longitude` requested. | [optional]
**use_uae_addressing_system** | Option<**bool**> | Regional setting \"Use the UAE Addressing System\".                Available if `UseUaeAddressingSystem` requested. | [optional]
**country_phone_code** | Option<**String**> | Country dialing code.                Available if `CountryPhoneCode` requested. | [optional]
**marketing_source_required_in_delivery** | Option<**bool**> | Require mandatory marketing source input when creating a delivery.                Available if `MarketingSourceRequiredInDelivery` requested. | [optional]
**default_delivery_city_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Default delivery city.                Available if `DefaultDeliveryCityId` requested. | [optional]
**delivery_city_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Delivery cities.                Available if `DeliveryCityIds` requested. | [optional]
**delivery_service_type** | Option<[**models::OrderDeliverySettingsServiceType**](OrderDeliverySettingsServiceType.md)> | Delivery type.                Available if `DeliveryServiceType` requested. | [optional]
**delivery_order_payment_settings** | Option<[**models::OrderDeliveryOrderPaymentSettings**](OrderDeliveryOrderPaymentSettings.md)> | Delivery order payment settings.                Available if `DeliveryOrderPaymentSettings` requested. | [optional]
**default_call_center_payment_type_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Default payment type for CallCenter.                Available if `DefaultCallCenterPaymentTypeId` requested. | [optional]
**order_item_comment_enabled** | Option<**bool**> | Allow text comments for order items (in all restaurant sections).                Available if `OrderItemCommentEnabled` requested. | [optional]
**is_confirmation_enabled** | Option<**bool**> | Determines whether to use delivery confirmation.                Available if `IsConfirmationEnabled` requested. | [optional]
**confirm_allowed_interval_in_minutes** | Option<**i32**> | Confirm orders time interval.                Available if `ConfirmAllowedIntervalInMinutes` requested. | [optional]
**address_lookup** | Option<[**Vec<models::AddressHintsServiceType>**](AddressHintsServiceType.md)> | Available address lookup services.                Available if `AddressLookup` requested. | [optional]
**use_business_hours_and_mapping** | Option<**bool**> | Determines whether the organization use a business hours and mapping settings. | [optional]
**currency_iso_name** | Option<**String**> | ISO currency code (for example: RUB, USD, EUR). | [optional]
**external_data** | Option<[**Vec<models::ExternalData>**](ExternalData.md)> | Organization`s external data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


