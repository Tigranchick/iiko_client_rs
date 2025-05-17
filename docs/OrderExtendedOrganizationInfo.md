# OrderExtendedOrganizationInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country** | Option<**String**> | Country. | 
**restaurant_address** | Option<**String**> | Restaurant address. | 
**latitude** | **f64** | Latitude. | 
**longitude** | **f64** | Longitude. | 
**use_uae_addressing_system** | **bool** | Regional setting \"Use the UAE Addressing System\". | 
**version** | **String** | RMS version. | 
**currency_iso_name** | Option<**String**> | ISO currency code (for example: RUB, USD, EUR). | 
**currency_minimum_denomination** | Option<**f64**> | Value rounding of position. | 
**country_phone_code** | Option<**String**> | Country dialing code. | 
**marketing_source_required_in_delivery** | Option<**bool**> | Require mandatory marketing source input when creating a delivery. | 
**default_delivery_city_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Default delivery city. | 
**delivery_city_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Delivery cities. | 
**delivery_service_type** | Option<[**models::OrderDeliverySettingsServiceType**](OrderDeliverySettingsServiceType.md)> | Delivery type. | 
**delivery_order_payment_settings** | Option<[**models::OrderDeliveryOrderPaymentSettings**](OrderDeliveryOrderPaymentSettings.md)> | Delivery order payment settings. | [optional]
**default_call_center_payment_type_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Default payment type for CallCenter. | 
**order_item_comment_enabled** | Option<**bool**> | Allow text comments for order items (in all restaurant sections). | 
**inn** | Option<**String**> | Restaurant`s INN (Taxpayer Identification Number). | 
**address_format_type** | [**models::OrderAddressFormatType**](OrderAddressFormatType.md) | Address format type. | 
**is_confirmation_enabled** | Option<**bool**> | Determines whether to use delivery confirmation. | 
**confirm_allowed_interval_in_minutes** | Option<**i32**> | Confirm orders time interval. | 
**is_cloud** | **bool** | Determines whether organization is hosted in iikoCloud. | 
**is_anonymous_guests_allowed** | Option<**bool**> | If the store allows orders for anonymous guests, then it is not necessary to transfer  information about the guest as part of the delivery order. You can only transfer  the phone number and optionally name of the guest, which will not be stored in the guest base  and will only be used for the delivery of a current delivery order. | [optional]
**address_lookup** | [**Vec<models::AddressHintsServiceType>**](AddressHintsServiceType.md) | Available address lookup services. | 
**response_type** | **String** |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**name** | Option<**String**> | Organization name. | 
**code** | Option<**String**> | Organization`s code. | [optional]
**external_data** | Option<[**Vec<models::ExternalData>**](ExternalData.md)> | Organization`s external data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


