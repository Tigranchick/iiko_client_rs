# GetAllowedRestrictionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization ID. Deprecated, use \"organizationIds\". | [optional]
**organization_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Organization IDs.                Can be obtained by `/api/1/organizations` operation. | [optional]
**delivery_address** | Option<[**models::RestrictionsAddress**](RestrictionsAddress.md)> | Delivery address. | [optional]
**order_location** | Option<[**models::OrderLocation**](OrderLocation.md)> | Order location. | [optional]
**order_items** | Option<[**Vec<models::RestrictionsOrderItem>**](RestrictionsOrderItem.md)> | Order list. | [optional]
**is_courier_delivery** | **bool** | Type of delivery service. | 
**delivery_date** | Option<**String**> | Delivery date (Local for delivery terminal). | [optional]
**delivery_sum** | Option<**f64**> | Sum. | [optional]
**discount_sum** | Option<**f64**> | Discounts sum. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


