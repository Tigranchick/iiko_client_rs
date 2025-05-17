# RequestOrdersByDeliveryDateAndStatusRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**delivery_date_from** | **String** | Order delivery date (Local for delivery terminal). Lower limit.                The guaranteed order availability  is the last 7 days. To access earlier orders, use the `/api/1/deliveries/history/by_delivery_date_and_phone` method. | 
**delivery_date_to** | Option<**String**> | Order delivery date (Local for delivery terminal). Upper limit. | [optional]
**statuses** | Option<[**Vec<models::DeliveryStatus>**](DeliveryStatus.md)> | Allowed order statuses. | [optional]
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**courier_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of driver IDs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


